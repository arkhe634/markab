use crate::{
	repetition_parser::{
		RepetitionParserError,
		RepetitionParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct RepetitionParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P,
	min: usize,
	max: usize,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> RepetitionParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P, min: usize, max: usize) -> Self
	{
		Self {
			requirement,
			min,
			max,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for RepetitionParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	type Error = RepetitionParserError<'a, 'b, P>;
	type Output = Vec<P::Output>;
	type Requirement = RepetitionParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let mut result = vec![];
		for i in 0..self.min
		{
			let res: <P as Parser>::Output = self.requirement.parse(src, pos).map_err(|err| {
				*pos = from;
				RepetitionParserError::new(from, self.requirement(None), i, err)
			})?;
			result.push(res)
		}
		for _ in self.min..self.max
		{
			if let Ok(res) = self.requirement.parse(src, pos)
			{
				result.push(res)
			}
			else
			{
				break;
			}
		}
		Ok(result)
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		for i in 0..self.min
		{
			if let Err(err) = self.requirement.parse(src, pos)
			{
				*pos = from;
				return Some(RepetitionParserError::new(
					from,
					self.requirement(None),
					i,
					err,
				));
			}
		}
		for _ in self.min..self.max
		{
			if let Err(_) = self.requirement.parse(src, pos)
			{
				break;
			}
		}
		None
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		RepetitionParserRequirement::new(self.requirement.requirement(None), self.min, self.max)
	}
}
