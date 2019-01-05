use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct MapParserRequirement<R>
where
	R: Debug + Display,
{
	requirement: R,
}

impl<R> MapParserRequirement<R>
where
	R: Debug + Display,
{
	pub fn new(requirement: R) -> Self
	{
		Self { requirement }
	}

	pub fn requirement(&self) -> &R
	{
		&self.requirement
	}
}

impl<R> Display for MapParserRequirement<R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> mapped", self.requirement)
	}
}
