use crate::Error;
use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

#[derive(Debug)]
pub struct ParseableParserError<'a, 'b, E>
where
	E: Error<'a, 'b>,
{
	from: usize,
	requirement: &'a str,
	cause: E,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, E> ParseableParserError<'a, 'b, E>
where
	E: Error<'a, 'b>,
{
	pub fn new(from: usize, requirement: &'a str, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, E> Error<'a, 'b> for ParseableParserError<'a, 'b, E>
where
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

impl<'a, 'b, E> Display for ParseableParserError<'a, 'b, E>
where
	E: Error<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
