use crate::{
	and_parser::{
		AndParserError,
		AndParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
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

impl<'a, 'b, P> Parser<'a, 'b> for AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	type Error = AndParserError<P::Requirement, P::Error>;
	type Output = P::Output;
	type Requirement = AndParserRequirement<P::Requirement>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
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
