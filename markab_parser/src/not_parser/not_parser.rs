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

#[derive(Debug)]
pub struct NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
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

impl<'a, 'b, P> Parser<'a, 'b> for NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	type Error = NotParserError<'a, 'b, P>;
	type Output = P::Error;
	type Requirement = NotParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		match self.requirement.parse(src, pos)
		{
			Ok(res) =>
			{
				*pos = from;
				Err(NotParserError::new(from, self.requirement(None), res))
			}
			Err(err) => Ok(err),
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		NotParserRequirement::new(self.requirement.requirement(None))
	}
}

#[derive(Debug)]
pub struct NotParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> NotParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}
}

impl<'a, 'b, P> Display for NotParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "!{}", self.requirement)
	}
}

#[derive(Debug)]
pub struct NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	from: usize,
	requirement: NotParserRequirement<'a, 'b, P>,
	cause: P::Output,
}

impl<'a, 'b, P> NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	pub fn new(from: usize, requirement: NotParserRequirement<'a, 'b, P>, cause: P::Output)
		-> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}

	pub fn cause(&self) -> &P::Output
	{
		&self.cause
	}
}

impl<'a, 'b, P> Error<'a, 'b> for NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
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
		write!(f, "success to parse")
	}

	fn causes(&self, _: &mut Formatter, _: usize) -> FmtResult
	{
		Ok(())
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

impl<'a, 'b, P> Display for NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}

pub fn not<'a, 'b, P>(parser: P) -> NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	NotParser::new(parser)
}
