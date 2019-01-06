use crate::{
	not_parser::{
		NotParserError,
		NotParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NotParser<'a, P>
where
	P: Parser<'a>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
}

impl<'a, P> NotParser<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(requirement: P) -> Self
	{
		Self {
			requirement,
			_a: PhantomData,
		}
	}
}

impl<'a, P> Parser<'a> for NotParser<'a, P>
where
	P: Parser<'a>,
{
	type Error = NotParserError<P::Requirement, P::Output>;
	type Output = P::Error;
	type Requirement = NotParserRequirement<P::Requirement>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		match self.requirement.parse(src, pos)
		{
			Ok(res) =>
			{
				*pos = from;
				Err(NotParserError::new(from, self.requirement(None), res))
			}
			Err(err) => Ok(err),
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		NotParserRequirement::new(self.requirement.requirement(None))
	}
}
