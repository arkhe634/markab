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

pub struct SequenceParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P,
	second: Q,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, Q> SequenceParser<'a, 'b, P, Q>
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

impl<'a, 'b, P, Q> Parser<'a, 'b> for SequenceParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	type Error = SequenceParserError<'a, 'b, P, Q>;
	type Output = (P::Output, Q::Output);
	type Requirement = SequenceParserRequirement<'a, 'b, P, Q>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let first = self
			.first
			.parse(src, pos)
			.map_err(|err| SequenceParserError::new(from, self.requirement(None), Left(err)))?;
		let second = self.second.parse(src, pos).map_err(|err| {
			*pos = from;
			SequenceParserError::new(from, self.requirement(None), Right(err))
		})?;
		Ok((first, second))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		SequenceParserRequirement::new(self.first.requirement(None), self.second.requirement(None))
	}
}

pub struct SequenceParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P::Requirement,
	second: Q::Requirement,
}

impl<'a, 'b, P, Q> SequenceParserRequirement<'a, 'b, P, Q>
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

impl<'a, 'b, P, Q> Display for SequenceParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{} {}", self.first, self.second)
	}
}

pub struct SequenceParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	from: usize,
	requirement: SequenceParserRequirement<'a, 'b, P, Q>,
	cause: Either<P::Error, Q::Error>,
}

impl<'a, 'b, P, Q> SequenceParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: SequenceParserRequirement<'a, 'b, P, Q>,
		cause: Either<P::Error, Q::Error>,
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P, Q> Error<'a, 'b> for SequenceParserError<'a, 'b, P, Q>
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
		match &self.cause
		{
			Left(_) => write!(f, "failed to parse {}", self.requirement.first()),
			Right(_) => write!(f, "failed to parse {}", self.requirement.second()),
		}
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		match &self.cause
		{
			Left(err) => err.print(f, depth),
			Right(err) => err.print(f, depth),
		}
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
}
