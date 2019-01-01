use crate::Parser;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	requirement: P::Requirement,
	generated: Option<Q::Requirement>,
}

impl<'a, 'b, P, Q> GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement, generated: Option<Q::Requirement>) -> Self
	{
		Self {
			requirement,
			generated,
		}
	}

	pub fn first(&self) -> &P::Requirement
	{
		&self.requirement
	}

	pub fn second(&self) -> Option<&Q::Requirement>
	{
		self.generated.as_ref()
	}
}

impl<'a, 'b, P, Q> Display for GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
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
