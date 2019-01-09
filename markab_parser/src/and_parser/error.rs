use crate::{
	and_parser::AndParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct AndParserError<'a, P>
where
	P: Parser<'a>,
{
	from: usize,
	requirement: AndParserRequirement<'a, P>,
	cause: P::Error,
}

impl<'a, P> AndParserError<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(from: usize, requirement: AndParserRequirement<'a, P>, cause: P::Error) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, P> Error for AndParserError<'a, P>
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
		self.cause.print(f, depth)
	}
}
