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
	E: Error,
{
	from: usize,
	requirement: MapParserRequirement<'a, 'b, R>,
	cause: E,
}

impl<'a, 'b, R, E> MapParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
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

impl<'a, 'b, R, E> Error for MapParserError<'a, 'b, R, E>
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
		self.cause.print(f, depth)
	}

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.causes(f, depth)
	}
}

impl<'a, 'b, R, E> Display for MapParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
