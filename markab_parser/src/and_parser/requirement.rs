use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct AndParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	requirement: P::Requirement,
}

impl<'a, P> AndParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}
}

impl<'a, P> Display for AndParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "&{}", self.requirement)
	}
}
