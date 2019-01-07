use crate::{
	repetition_parser::RepetitionParserRequirement,
	Error,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct RepetitionParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: RepetitionParserRequirement<R>,
	found: usize,
	cause: E,
}

impl<R, E> RepetitionParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(
		from: usize,
		requirement: RepetitionParserRequirement<R>,
		found: usize,
		cause: E,
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

impl<R, E> Error for RepetitionParserError<R, E>
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
		write!(f, "succeed in parsing only {} time(s)", self.found)
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.cause.print(f, depth)
	}
}

impl<R, E> Display for RepetitionParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
