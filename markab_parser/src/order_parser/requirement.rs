use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct OrderParserRequirement<R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	first: R1,
	second: R2,
}

impl<R1, R2> OrderParserRequirement<R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	pub fn new(first: R1, second: R2) -> Self
	{
		Self { first, second }
	}

	pub fn first(&self) -> &R1
	{
		&self.first
	}

	pub fn second(&self) -> &R2
	{
		&self.second
	}
}

impl<R1, R2> Display for OrderParserRequirement<R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{} / {}", self.first, self.second)
	}
}
