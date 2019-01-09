use crate::{
	stringify_parser::StringifyParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct StringifyParserError<'a, P>
where
	P: Parser<'a>,
{
	from: usize,
	requirement: StringifyParserRequirement<'a, P>,
	err: P::Error,
}

impl<'a, P> StringifyParserError<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(from: usize, requirement: StringifyParserRequirement<'a, P>, err: P::Error) -> Self
	{
		Self {
			from,
			requirement,
			err,
		}
	}
}

impl<'a, P> Error for StringifyParserError<'a, P>
where
	P: Parser<'a>,
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

impl<'a, P> Display for StringifyParserError<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
