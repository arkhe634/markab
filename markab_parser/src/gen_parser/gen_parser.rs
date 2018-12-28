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

pub struct GenParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(&P::Output) -> Q,
	Q: Parser<'a, 'b>,
{
	requirement: P,
	generator: F,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, F, Q> GenParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(&P::Output) -> Q,
	Q: Parser<'a, 'b>,
{
	pub fn new(requirement: P, generator: F) -> Self
	{
		Self {
			requirement,
			generator,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P, F, Q> Parser<'a, 'b> for GenParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(&P::Output) -> Q,
	Q: Parser<'a, 'b>,
{
	type Error = GenParserError<'a, 'b, P, Q>;
	type Output = (P::Output, Q::Output);
	type Requirement = GenParserRequirement<'a, 'b, P, Q>;
	type RequirementContext = Q;

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let res1 = self
			.requirement
			.parse(src, pos)
			.map_err(|err| GenParserError::new(from, self.requirement(None), Left(err)))?;
		let parser = (self.generator)(&res1);
		let res2 = parser.parse(src, pos).map_err(|err| {
			GenParserError::new(from, self.requirement(Some(&parser)), Right(err))
		})?;
		Ok((res1, res2))
	}

	fn requirement(&self, context: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		if let Some(context) = context
		{
			GenParserRequirement::new(
				self.requirement.requirement(None),
				Some(context.requirement(None)),
			)
		}
		else
		{
			GenParserRequirement::new(self.requirement.requirement(None), None)
		}
	}
}

pub struct GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	requirement: P::Requirement,
	generated: Option<Q::Requirement>,
}

impl<'a, 'b, P, Q> GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement, generated: Option<Q::Requirement>) -> Self
	{
		Self {
			requirement,
			generated,
		}
	}

	pub fn first(&self) -> &P::Requirement
	{
		&self.requirement
	}

	pub fn second(&self) -> Option<&Q::Requirement>
	{
		self.generated.as_ref()
	}
}

impl<'a, 'b, P, Q> Display for GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		if let Some(generated) = &self.generated
		{
			write!(f, "{} {}", self.requirement, generated)
		}
		else
		{
			write!(f, "({}) -> gen", self.requirement)
		}
	}
}

pub struct GenParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	from: usize,
	requirement: GenParserRequirement<'a, 'b, P, Q>,
	cause: Either<P::Error, Q::Error>,
}

impl<'a, 'b, P, Q> GenParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: GenParserRequirement<'a, 'b, P, Q>,
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

impl<'a, 'b, P, Q> Error<'a, 'b> for GenParserError<'a, 'b, P, Q>
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
			Right(_) => write!(f, "failed to parse {}", self.requirement.second().unwrap()),
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

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.print(f, depth)
	}
}
