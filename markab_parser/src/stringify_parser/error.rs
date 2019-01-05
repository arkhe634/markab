use crate::{
	stringify_parser::StringifyParserRequirement,
	Error,
};
use std::{
	fmt::{
		Debug,
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

#[derive(Debug)]
pub struct StringifyParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: StringifyParserRequirement<'a, 'b, R>,
	err: E,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R, E> StringifyParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(from: usize, requirement: StringifyParserRequirement<'a, 'b, R>, err: E) -> Self
	{
		Self {
			from,
			requirement,
			err,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, R, E> Error for StringifyParserError<'a, 'b, R, E>
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

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
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
}

impl<'a, 'b, R, E> Display for StringifyParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
