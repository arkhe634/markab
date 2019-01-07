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
pub struct MapParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	from: usize,
	requirement: MapParserRequirement<R>,
	cause: E,
}

impl<R, E> MapParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	pub fn new(from: usize, requirement: MapParserRequirement<R>, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<R, E> Error for MapParserError<R, E>
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

impl<R, E> Display for MapParserError<R, E>
where
	R: Debug + Display,
	E: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
