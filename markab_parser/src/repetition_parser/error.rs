use crate::{
	repetition_parser::RepetitionParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct RepetitionParserError<'a, P>
where
	P: Parser<'a>,
{
	from: usize,
	requirement: RepetitionParserRequirement<'a, P>,
	found: usize,
	cause: P::Error,
}

impl<'a, P> RepetitionParserError<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(
		from: usize,
		requirement: RepetitionParserRequirement<'a, P>,
		found: usize,
		cause: P::Error,
	) -> Self
	{
		Self {
			from,
			requirement,
			found,
			cause,
		}
	}
}

impl<'a, P> Error for RepetitionParserError<'a, P>
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
		write!(f, "succeed in parsing only {} time(s)", self.found)
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.cause.print(f, depth)
	}
}

impl<'a, P> Display for RepetitionParserError<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
