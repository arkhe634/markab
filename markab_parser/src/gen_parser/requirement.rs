use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct GenParserRequirement<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	requirement: P1::Requirement,
	generated: Option<P2::Requirement>,
}

impl<'a, P1, P2> GenParserRequirement<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	pub fn new(requirement: P1::Requirement, generated: Option<P2::Requirement>) -> Self
	{
		Self {
			requirement,
			generated,
		}
	}

	pub fn first(&self) -> &P1::Requirement
	{
		&self.requirement
	}

	pub fn second(&self) -> Option<&P2::Requirement>
	{
		self.generated.as_ref()
	}
}

impl<'a, P1, P2> Display for GenParserRequirement<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		if let Some(generated) = &self.generated
		{
			write!(f, "{} {}", self.requirement, generated)
		}
		else
		{
			write!(f, "({}) -> gen", self.requirement)
		}
	}
}
