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

pub struct ParseableParser<'a, P>
where
	P: Parseable<'a>,
{
	_a: PhantomData<&'a ()>,
	_p: PhantomData<P>,
}

impl<'a, P> ParseableParser<'a, P>
where
	P: Parseable<'a>,
{
	pub fn new() -> Self
	{
		Self {
			_a: PhantomData,
			_p: PhantomData,
		}
	}
}

impl<'a, P> Debug for ParseableParser<'a, P>
where
	P: Parseable<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		f.debug_struct("ParseableParser")
			.field("type", &P::name())
			.finish()
	}
}

impl<'a, P> Parser<'a> for ParseableParser<'a, P>
where
	P: Parseable<'a>,
{
	type Error = ParseableParserError<'a, P::Error>;
	type Output = P::Output;
	type Requirement = &'a str;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		P::parse(src, pos).map_err(|err| ParseableParserError::new(from, P::name(), err))
	}

	fn skip(&self, src: &'a str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		P::skip(src, pos).map(|err| ParseableParserError::new(from, P::name(), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		P::name()
	}
}
