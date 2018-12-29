use crate::{
	Error,
	Parser,
};
use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P) -> Self
	{
		Self {
			requirement,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	type Error = AndParserError<'a, 'b, P>;
	type Output = P::Output;
	type Requirement = AndParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.parse(src, pos)
			.map(|res| {
				*pos = from;
				res
			})
			.map_err(|err| AndParserError::new(from, self.requirement(None), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		AndParserRequirement::new(self.requirement.requirement(None))
	}
}

pub struct AndParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> AndParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}
}

impl<'a, 'b, P> Display for AndParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "&{}", self.requirement)
	}
}

pub struct AndParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	from: usize,
	requirement: AndParserRequirement<'a, 'b, P>,
	cause: P::Error,
}

impl<'a, 'b, P> AndParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(from: usize, requirement: AndParserRequirement<'a, 'b, P>, cause: P::Error) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P> Error<'a, 'b> for AndParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
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
		self.cause.print(f, depth)
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

pub fn and<'a, 'b, P>(parser: P) -> AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	AndParser::new(parser)
}
