use crate::{
	stringify_parser::StringifyParserRequirement,
	Error,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct StringifyParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: StringifyParserRequirement<R>,
	err: E,
}

impl<R, E> StringifyParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(from: usize, requirement: StringifyParserRequirement<R>, err: E) -> Self
	{
		Self {
			from,
			requirement,
			err,
		}
	}
}

impl<R, E> Error for StringifyParserError<R, E>
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
		self.err.print(f, depth)
	}

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.causes(f, depth)
	}
}

impl<R, E> Display for StringifyParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
