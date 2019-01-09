use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct OrderParserRequirement<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	first: P1::Requirement,
	second: P2::Requirement,
}

impl<'a, P1, P2> OrderParserRequirement<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	pub fn new(first: P1::Requirement, second: P2::Requirement) -> Self
	{
		Self { first, second }
	}

	pub fn first(&self) -> &P1::Requirement
	{
		&self.first
	}

	pub fn second(&self) -> &P2::Requirement
	{
		&self.second
	}
}

impl<'a, P1, P2> Display for OrderParserRequirement<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{} / {}", self.first, self.second)
	}
}
