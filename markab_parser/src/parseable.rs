use crate::{
	parseable_parser::ParseableParser,
	Error,
};

pub trait Parseable<'a>
{
	type Output;

	fn parse(src: &'a str, pos: &mut usize) -> Result<Self::Output, Box<'a + Error>>;

	fn skip(src: &'a str, pos: &mut usize) -> Option<Box<'a + Error>>
	{
		Self::parse(src, pos).err()
	}

	fn name() -> &'a str;

	fn get_parser() -> ParseableParser<'a, Self>
	where
		Self: Sized,
	{
		ParseableParser::new()
	}
}
