use crate::{
	and_parser::{
		AndParserError,
		AndParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct AndParser<'a, P>
where
	P: Parser<'a>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
}

impl<'a, P> AndParser<'a, P>
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

impl<'a, P> Parser<'a> for AndParser<'a, P>
where
	P: Parser<'a>,
{
	type Error = AndParserError<P::Requirement, P::Error>;
	type Output = P::Output;
	type Requirement = AndParserRequirement<P::Requirement>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.parse(src, pos)
			.map(|res| {
				*pos = from;
				res
			})
			.map_err(|err| AndParserError::new(from, self.requirement(None), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		AndParserRequirement::new(self.requirement.requirement(None))
	}
}
