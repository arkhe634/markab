use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct NotParserRequirement<R>
where
	R: Debug + Display,
{
	requirement: R,
}

impl<R> NotParserRequirement<R>
where
	R: Debug + Display,
{
	pub fn new(requirement: R) -> Self
	{
		Self { requirement }
	}
}

impl<R> Display for NotParserRequirement<R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "!{}", self.requirement)
	}
}
