use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct AndParserRequirement<R>
where
	R: Debug + Display,
{
	requirement: R,
}

impl<R> AndParserRequirement<R>
where
	R: Debug + Display,
{
	pub fn new(requirement: R) -> Self
	{
		Self { requirement }
	}
}

impl<R> Display for AndParserRequirement<R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "&{}", self.requirement)
	}
}
