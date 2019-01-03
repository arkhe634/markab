use crate::{
	map_parser::MapParserRequirement,
	Error,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct MapParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error<'a, 'b>,
{
	from: usize,
	requirement: MapParserRequirement<'a, 'b, R>,
	cause: E,
}

impl<'a, 'b, R, E> MapParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error<'a, 'b>,
{
	pub fn new(from: usize, requirement: MapParserRequirement<'a, 'b, R>, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, R, E> Error<'a, 'b> for MapParserError<'a, 'b, R, E>
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

impl<'a, 'b, R, E> Display for MapParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
