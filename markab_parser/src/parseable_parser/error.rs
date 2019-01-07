use crate::Error;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

pub struct ParseableParserError<'a>
{
	from: usize,
	requirement: &'a str,
	cause: Box<Error>,
}

impl<'a> ParseableParserError<'a>
{
	pub fn new(from: usize, requirement: &'a str, cause: Box<Error>) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a> Error for ParseableParserError<'a>
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

impl<'a> Display for ParseableParserError<'a>
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
