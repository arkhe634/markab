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
pub struct RepetitionParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: RepetitionParserRequirement<'a, 'b, R>,
	found: usize,
	cause: E,
}

impl<'a, 'b, R, E> RepetitionParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(
		from: usize,
		requirement: RepetitionParserRequirement<'a, 'b, R>,
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

impl<'a, 'b, R, E> Error for RepetitionParserError<'a, 'b, R, E>
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

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		for _ in 0..depth
		{
			write!(f, "\t")?;
		}
		write!(f, "at position ")?;
		self.from(f)?;
		write!(f, " required ")?;
		self.requirement(f)?;
		write!(f, " but ")?;
		self.result(f)?;
		write!(f, ".\n")?;
		self.causes(f, depth + 1)
	}

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.print(f, depth)
	}
}

impl<'a, 'b, R, E> Display for RepetitionParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
