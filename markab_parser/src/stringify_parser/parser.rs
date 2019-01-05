use crate::{
	stringify_parser::{
		StringifyParserError,
		StringifyParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct StringifyParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> StringifyParser<'a, 'b, P>
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

impl<'a, 'b, P> Parser<'a, 'b> for StringifyParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	type Error = StringifyParserError<P::Requirement, P::Error>;
	type Output = &'b str;
	type Requirement = StringifyParserRequirement<P::Requirement>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
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
