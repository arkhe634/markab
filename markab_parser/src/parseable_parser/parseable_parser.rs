use crate::{
	Error,
	Parseable,
	Parser,
};
use std::{
	fmt::{
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
	_p: PhantomData<P>,
}

impl<'a, 'b, P> ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	pub fn new() -> Self
	{
		Self {
			_a: PhantomData,
			_b: PhantomData,
			_p: PhantomData,
		}
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	type Error = ParseableParserError<'a, 'b, P>;
	type Output = P::Output;
	type Requirement = &'a str;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		P::parse(src, pos).map_err(|err| ParseableParserError::new(from, P::name(), err))
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		P::skip(src, pos).map(|err| ParseableParserError::new(from, P::name(), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		P::name()
	}
}

pub struct ParseableParserError<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	from: usize,
	requirement: &'a str,
	cause: P::Error,
}

impl<'a, 'b, P> ParseableParserError<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	pub fn new(from: usize, requirement: &'a str, cause: P::Error) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P> Error<'a, 'b> for ParseableParserError<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
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
