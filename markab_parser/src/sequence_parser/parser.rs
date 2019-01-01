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
pub struct SequenceParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P,
	second: Q,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, Q> SequenceParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(first: P, second: Q) -> Self
	{
		Self {
			first,
			second,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P, Q> Parser<'a, 'b> for SequenceParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	type Error = SequenceParserError<'a, 'b, P, Q>;
	type Output = (P::Output, Q::Output);
	type Requirement = SequenceParserRequirement<'a, 'b, P, Q>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
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

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
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
