use crate::{
	parseable_parser::ParseableParser,
	Error,
};

pub trait Parseable<'a, 'b>
{
	type Error: Error<'a, 'b>;
	type Output: 'b;

	fn parse(src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn name() -> &'a str;

	fn get_parser() -> ParseableParser<'a, 'b, Self>
	where
		Self: Sized,
	{
		ParseableParser::new()
	}
}
