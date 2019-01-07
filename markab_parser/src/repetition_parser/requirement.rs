use std::{
	fmt::{
		Debug,
		Display,
		Formatter,
		Result as FmtResult,
	},
	usize::MAX,
};

#[derive(Debug)]
pub struct RepetitionParserRequirement<R>
where
	R: Debug + Display,
{
	requirement: R,
	min: usize,
	max: usize,
}

impl<R> RepetitionParserRequirement<R>
where
	R: Debug + Display,
{
	pub fn new(requirement: R, min: usize, max: usize) -> Self
	{
		Self {
			requirement,
			min,
			max,
		}
	}
}

impl<R> Display for RepetitionParserRequirement<R>
where
	R: Debug + Display,
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
