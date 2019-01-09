use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct MapParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	requirement: P::Requirement,
}

impl<'a, P> MapParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}

	pub fn requirement(&self) -> &P::Requirement
	{
		&self.requirement
	}
}

impl<'a, P> Display for MapParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> mapped", self.requirement)
	}
}
