use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct SequenceParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P::Requirement,
	second: Q::Requirement,
}

impl<'a, 'b, P, Q> SequenceParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(first: P::Requirement, second: Q::Requirement) -> Self
	{
		Self { first, second }
	}

	pub fn first(&self) -> &P::Requirement
	{
		&self.first
	}

	pub fn second(&self) -> &Q::Requirement
	{
		&self.second
	}
}

impl<'a, 'b, P, Q> Display for SequenceParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{} {}", self.first, self.second)
	}
}
