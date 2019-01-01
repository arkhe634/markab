use crate::{
	not_parser::{
		NotParserError,
		NotParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	pub fn new(requirement: P) -> Self
	{
		Self {
			requirement,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	type Error = NotParserError<'a, 'b, P>;
	type Output = P::Error;
	type Requirement = NotParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
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
