use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct StringifyParserRequirement<R>
where
	R: Debug + Display,
{
	requirement: R,
}

impl<R> StringifyParserRequirement<R>
where
	R: Debug + Display,
{
	pub fn new(requirement: R) -> Self
	{
		Self { requirement }
	}
}

impl<R> Display for StringifyParserRequirement<R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> stringify", self.requirement)
	}
}
