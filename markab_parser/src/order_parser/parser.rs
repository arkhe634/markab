use crate::{
	order_parser::{
		OrderParserError,
		OrderParserRequirement,
	},
	Parser,
};
use either::{
	Either,
	Left,
	Right,
};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct OrderParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	first: P,
	second: Q,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, Q> OrderParser<'a, 'b, P, Q>
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

impl<'a, 'b, P, Q> Parser<'a, 'b> for OrderParser<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	type Error = OrderParserError<'a, 'b, P, Q>;
	type Output = Either<P::Output, Q::Output>;
	type Requirement = OrderParserRequirement<'a, 'b, P, Q>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let first = match self.first.parse(src, pos)
		{
			Ok(res) => return Ok(Left(res)),
			Err(err) => err,
		};
		let second = match self.second.parse(src, pos)
		{
			Ok(res) => return Ok(Right(res)),
			Err(err) => err,
		};
		Err(OrderParserError::new(
			from,
			self.requirement(None),
			(first, second),
		))
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		self.first.skip(src, pos).and_then(|first| {
			self.second
				.skip(src, pos)
				.map(|second| OrderParserError::new(from, self.requirement(None), (first, second)))
		})
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		OrderParserRequirement::new(self.first.requirement(None), self.second.requirement(None))
	}
}
