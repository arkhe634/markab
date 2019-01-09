use crate::{
	sequence_parser::{
		SequenceParserError,
		SequenceParserRequirement,
	},
	Parser,
};
use either::{
	Left,
	Right,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SequenceParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	first: P1,
	second: P2,
	_a: PhantomData<&'a ()>,
}

impl<'a, P1, P2> SequenceParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	pub fn new(first: P1, second: P2) -> Self
	{
		Self {
			first,
			second,
			_a: PhantomData,
		}
	}
}

impl<'a, P1, P2> Parser<'a> for SequenceParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	type Error = SequenceParserError<'a, P1, P2>;
	type Output = (P1::Output, P2::Output);
	type Requirement = SequenceParserRequirement<'a, P1, P2>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let first = self
			.first
			.parse(src, pos)
			.map_err(|err| SequenceParserError::new(from, self.requirement(None), Left(err)))?;
		let second = self.second.parse(src, pos).map_err(|err| {
			*pos = from;
			SequenceParserError::new(from, self.requirement(None), Right(err))
		})?;
		Ok((first, second))
	}

	fn skip(&self, src: &'a str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		self.first
			.skip(src, pos)
			.map(|err| SequenceParserError::new(from, self.requirement(None), Left(err)))
			.or_else(|| {
				self.second
					.skip(src, pos)
					.map(|err| SequenceParserError::new(from, self.requirement(None), Right(err)))
			})
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		SequenceParserRequirement::new(self.first.requirement(None), self.second.requirement(None))
	}
}
