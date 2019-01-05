use crate::{
	parseable_parser::ParseableParser,
	Error,
};

pub trait Parseable<'a, 'b>
{
	type Error: Error;
	type Output;

	fn parse(src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn skip(src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		Self::parse(src, pos).err()
	}

	fn name() -> &'a str;

	fn get_parser() -> ParseableParser<'a, 'b, Self>
	where
		Self: Sized,
	{
		ParseableParser::new()
	}
}
