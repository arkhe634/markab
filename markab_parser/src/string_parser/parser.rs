use crate::{
	string_parser::StringParserError,
	Parser,
};

#[derive(Debug)]
pub struct StringParser<'a>
{
	requirement: &'a str,
}

impl<'a> StringParser<'a>
{
	pub fn new(requirement: &'a str) -> Self
	{
		Self { requirement }
	}
}

impl<'a> Parser<'a> for StringParser<'a>
{
	type Error = StringParserError<'a>;
	type Output = &'a str;
	type Requirement = &'a str;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let len = self.requirement.len();
		if src[from..].starts_with(self.requirement)
		{
			*pos += len;
			Ok(&src[from..*pos])
		}
		else
		{
			Err(StringParserError::new(from, self.requirement(None), src))
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		self.requirement
	}
}
