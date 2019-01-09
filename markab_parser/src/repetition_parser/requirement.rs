use crate::Parser;
use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	usize::MAX,
};

#[derive(Debug)]
pub struct RepetitionParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	requirement: P::Requirement,
	min: usize,
	max: usize,
}

impl<'a, P> RepetitionParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(requirement: P::Requirement, min: usize, max: usize) -> Self
	{
		Self {
			requirement,
			min,
			max,
		}
	}
}

impl<'a, P> Display for RepetitionParserRequirement<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		match (self.min, self.max)
		{
			(0, 1) => write!(f, "{}?", self.requirement),
			(0, MAX) => write!(f, "{}*", self.requirement),
			(1, MAX) => write!(f, "{}+", self.requirement),
			(i, j) if i == j => write!(f, "{}{{{}}}", self.requirement, i),
			(i, j) => write!(f, "{}{{{},{}}}", self.requirement, i, j),
		}
	}
}
