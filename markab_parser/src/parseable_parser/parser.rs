use crate::{
	parseable_parser::ParseableParserError,
	Parseable,
	Parser,
};
use std::{
	fmt::{
		Debug,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
	_p: PhantomData<P>,
}

impl<'a, 'b, P> ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	pub fn new() -> Self
	{
		Self {
			_a: PhantomData,
			_b: PhantomData,
			_p: PhantomData,
		}
	}
}

impl<'a, 'b, P> Debug for ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		f.debug_struct("ParseableParser")
			.field("type", &P::name())
			.finish()
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for ParseableParser<'a, 'b, P>
where
	P: Parseable<'a, 'b>,
{
	type Error = ParseableParserError<'a, 'b, P>;
	type Output = P::Output;
	type Requirement = &'a str;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		P::parse(src, pos).map_err(|err| ParseableParserError::new(from, P::name(), err))
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		P::skip(src, pos).map(|err| ParseableParserError::new(from, P::name(), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		P::name()
	}
}
