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
	E: Error<'a, 'b>,
{
	from: usize,
	requirement: AndParserRequirement<'a, 'b, R>,
	cause: E,
}

impl<'a, 'b, R, E> AndParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error<'a, 'b>,
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

impl<'a, 'b, R, E> Error<'a, 'b> for AndParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error<'a, 'b>,
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
