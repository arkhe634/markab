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
	E: Error,
{
	from: usize,
	requirement: &'a str,
	cause: E,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, E> ParseableParserError<'a, 'b, E>
where
	E: Error,
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

impl<'a, 'b, E> Error for ParseableParserError<'a, 'b, E>
where
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

impl<'a, 'b, E> Display for ParseableParserError<'a, 'b, E>
where
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
