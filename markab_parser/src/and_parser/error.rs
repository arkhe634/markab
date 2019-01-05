use crate::{
	and_parser::AndParserRequirement,
	Error,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct AndParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: AndParserRequirement<'a, 'b, R>,
	cause: E,
}

impl<'a, 'b, R, E> AndParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(from: usize, requirement: AndParserRequirement<'a, 'b, R>, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, R, E> Error for AndParserError<'a, 'b, R, E>
where
	R: Debug + Display,
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
