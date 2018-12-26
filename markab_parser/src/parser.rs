use crate::Error;
use std::fmt::Display;

pub trait Parser<'a, 'b>
{
	type Error: Error<'a, 'b>;
	type Output: 'b;
	type Requirement: Display;
	type RequirementContext;

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn requirement(&self, context: Option<&Self::RequirementContext>) -> Self::Requirement;
}
