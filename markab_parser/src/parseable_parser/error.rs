use crate::Error;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct ParseableParserError<'a, E>
where
	E: Error,
{
	from: usize,
	requirement: &'a str,
	cause: E,
}

impl<'a, E> ParseableParserError<'a, E>
where
	E: Error,
{
	pub fn new(from: usize, requirement: &'a str, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, E> Error for ParseableParserError<'a, E>
where
	E: Error,
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

impl<'a, E> Display for ParseableParserError<'a, E>
where
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
