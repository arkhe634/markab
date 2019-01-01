use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct MapParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> MapParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
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

impl<'a, 'b, P> Display for MapParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> mapped", self.requirement)
	}
}
