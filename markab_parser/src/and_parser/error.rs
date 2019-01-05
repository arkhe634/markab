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
pub struct AndParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: AndParserRequirement<R>,
	cause: E,
}

impl<R, E> AndParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(from: usize, requirement: AndParserRequirement<R>, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<R, E> Error for AndParserError<R, E>
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
