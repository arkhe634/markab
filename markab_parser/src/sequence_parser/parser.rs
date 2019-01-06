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
pub struct SequenceParser<'a, P, Q>
where
	P: Parser<'a>,
	Q: Parser<'a>,
{
	first: P,
	second: Q,
	_a: PhantomData<&'a ()>,
}

impl<'a, P, Q> SequenceParser<'a, P, Q>
where
	P: Parser<'a>,
	Q: Parser<'a>,
{
	pub fn new(first: P, second: Q) -> Self
	{
		Self {
			first,
			second,
			_a: PhantomData,
		}
	}
}

impl<'a, P, Q> Parser<'a> for SequenceParser<'a, P, Q>
where
	P: Parser<'a>,
	Q: Parser<'a>,
{
	type Error = SequenceParserError<P::Requirement, Q::Requirement, P::Error, Q::Error>;
	type Output = (P::Output, Q::Output);
	type Requirement = SequenceParserRequirement<P::Requirement, Q::Requirement>;
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
