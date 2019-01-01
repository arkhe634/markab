use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct NotParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> NotParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}
}

impl<'a, 'b, P> Display for NotParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "!{}", self.requirement)
	}
}
