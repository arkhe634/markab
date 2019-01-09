use crate::{
	Error,
	Parseable,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

pub struct ParseableParserError<'a, P>
where
	P: Parseable<'a>,
{
	from: usize,
	requirement: &'a str,
	cause: P::Error,
}

impl<'a, P> ParseableParserError<'a, P>
where
	P: Parseable<'a>,
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

impl<'a, P> Error for ParseableParserError<'a, P>
where
	P: Parseable<'a>,
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
}

impl<'a, P> Display for ParseableParserError<'a, P>
where
	P: Parseable<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
