use crate::{
	equal::Equal,
	map_parser::MapParser,
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
pub struct OrderParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	first: P1,
	second: P2,
	_a: PhantomData<&'a ()>,
}

impl<'a, P1, P2> OrderParser<'a, P1, P2>
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

impl<'a, P1, P2> OrderParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
	P2::Output: Equal<P1::Output>,
{
	pub fn merge(self) -> MapParser<'a, Self, P1::Output>
	{
		self.map(&|res| {
			match res
			{
				Left(first) => first,
				Right(second) => second.ident(),
			}
		})
	}
}

impl<'a, P1, P2> Parser<'a> for OrderParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	type Error = OrderParserError<'a, P1, P2>;
	type Output = Either<P1::Output, P2::Output>;
	type Requirement = OrderParserRequirement<'a, P1, P2>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
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

	fn skip(&self, src: &'a str, pos: &mut usize) -> Result<(), Self::Error>
	{
		let from = *pos;
		match self.first.skip(src, pos)
		{
			Ok(()) => Ok(()),
			Err(first) =>
			{
				self.second.skip(src, pos).map_err(|second| {
					OrderParserError::new(from, self.requirement(None), (first, second))
				})
			}
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		OrderParserRequirement::new(self.first.requirement(None), self.second.requirement(None))
	}
}
