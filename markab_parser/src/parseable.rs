use crate::{
	parseable_parser::ParseableParser,
	Error,
};

pub trait Parseable<'a>
{
	type Error: Error;
	type Output;

	fn parse(src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn skip(src: &'a str, pos: &mut usize) -> Result<(), Self::Error>
	{
		Self::parse(src, pos).map(|_| ())
	}

	fn name() -> &'a str;

	fn get_parser() -> ParseableParser<'a, Self>
	where
		Self: Sized,
	{
		ParseableParser::new()
	}
}
