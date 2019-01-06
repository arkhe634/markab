use crate::{
	stringify_parser::{
		StringifyParserError,
		StringifyParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct StringifyParser<'a, P>
where
	P: Parser<'a>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
}

impl<'a, P> StringifyParser<'a, P>
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

impl<'a, P> Parser<'a> for StringifyParser<'a, P>
where
	P: Parser<'a>,
{
	type Error = StringifyParserError<P::Requirement, P::Error>;
	type Output = &'a str;
	type Requirement = StringifyParserRequirement<P::Requirement>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.skip(src, pos)
			.map_or(Ok(&src[from..*pos]), |err| {
				Err(StringifyParserError::new(from, self.requirement(None), err))
			})
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		StringifyParserRequirement::new(self.requirement.requirement(None))
	}
}
