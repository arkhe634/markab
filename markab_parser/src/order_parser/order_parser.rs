use crate::{
	Error,
	Parser,
};
use either::{
	Either,
	Left,
	Right,
};
use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct OrderParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P,
	second: Q,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, Q> OrderParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(first: P, second: Q) -> Self
	{
		Self {
			first,
			second,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P, Q> Parser<'a, 'b> for OrderParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	type Error = OrderParserError<'a, 'b, P, Q>;
	type Output = Either<P::Output, Q::Output>;
	type Requirement = OrderParserRequirement<'a, 'b, P, Q>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let first = match self.first.parse(src, pos)
		{
			Ok(res) => return Ok(Left(res)),
			Err(err) => err,
		};
		let second = match self.second.parse(src, pos)
		{
			Ok(res) => return Ok(Right(res)),
			Err(err) => err,
		};
		Err(OrderParserError::new(
			from,
			self.requirement(None),
			(first, second),
		))
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		self.first.skip(src, pos).and_then(|first| {
			self.second
				.skip(src, pos)
				.map(|second| OrderParserError::new(from, self.requirement(None), (first, second)))
		})
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		OrderParserRequirement::new(self.first.requirement(None), self.second.requirement(None))
	}
}

pub struct OrderParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P::Requirement,
	second: Q::Requirement,
}

impl<'a, 'b, P, Q> OrderParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(first: P::Requirement, second: Q::Requirement) -> Self
	{
		Self { first, second }
	}

	pub fn first(&self) -> &P::Requirement
	{
		&self.first
	}

	pub fn second(&self) -> &Q::Requirement
	{
		&self.second
	}
}

impl<'a, 'b, P, Q> Display for OrderParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{} {}", self.first, self.second)
	}
}

pub struct OrderParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	from: usize,
	requirement: OrderParserRequirement<'a, 'b, P, Q>,
	cause: (P::Error, Q::Error),
}

impl<'a, 'b, P, Q> OrderParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: OrderParserRequirement<'a, 'b, P, Q>,
		cause: (P::Error, Q::Error),
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P, Q> Error<'a, 'b> for OrderParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	fn from(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{}", self.from)
	}

	fn requirement(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{}", self.requirement)
	}

	fn result(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "failed to parse")
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.cause.0.print(f, depth)?;
		self.cause.1.print(f, depth)
	}

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		for _ in 0..depth
		{
			write!(f, "\t")?;
		}
		write!(f, "at position ")?;
		self.from(f)?;
		write!(f, " required ")?;
		self.requirement(f)?;
		write!(f, " but ")?;
		self.result(f)?;
		write!(f, ".\n")?;
		self.causes(f, depth + 1)
	}

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.print(f, depth)
	}
}
