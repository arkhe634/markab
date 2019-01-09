use crate::{
	repetition_parser::{
		RepetitionParserError,
		RepetitionParserRequirement,
	},
	Parser,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct RepetitionParser<'a, P>
where
	P: Parser<'a>,
{
	requirement: P,
	min: usize,
	max: usize,
	_a: PhantomData<&'a ()>,
}

impl<'a, P> RepetitionParser<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(requirement: P, min: usize, max: usize) -> Self
	{
		Self {
			requirement,
			min,
			max,
			_a: PhantomData,
		}
	}
}

impl<'a, P> Parser<'a> for RepetitionParser<'a, P>
where
	P: Parser<'a>,
{
	type Error = RepetitionParserError<'a, P>;
	type Output = Vec<P::Output>;
	type Requirement = RepetitionParserRequirement<'a, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
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

	fn skip(&self, src: &'a str, pos: &mut usize) -> Result<(), Self::Error>
	{
		let from = *pos;
		for i in 0..self.min
		{
			if let Err(err) = self.requirement.parse(src, pos)
			{
				*pos = from;
				return Err(RepetitionParserError::new(
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
		Ok(())
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		RepetitionParserRequirement::new(self.requirement.requirement(None), self.min, self.max)
	}
}
