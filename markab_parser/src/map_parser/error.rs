use crate::{
	map_parser::MapParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct MapParserError<'a, P>
where
	P: Parser<'a>,
{
	from: usize,
	requirement: MapParserRequirement<'a, P>,
	cause: P::Error,
}

impl<'a, P> MapParserError<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(from: usize, requirement: MapParserRequirement<'a, P>, cause: P::Error) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, P> Error for MapParserError<'a, P>
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

impl<'a, P> Display for MapParserError<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
