use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct AndParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> AndParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}
}

impl<'a, 'b, P> Display for AndParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "&{}", self.requirement)
	}
}
