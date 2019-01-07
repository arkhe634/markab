use crate::{
	character_parser::CharacterParserError,
	Parser,
};

#[derive(Debug)]
pub struct CharacterParser
{
	requirement: char,
}

impl CharacterParser
{
	pub fn new(requirement: char) -> Self
	{
		Self { requirement }
	}
}

impl<'a> Parser<'a> for CharacterParser
{
	type Error = CharacterParserError;
	type Output = &'a str;
	type Requirement = char;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let next = src[from..].chars().next();
		if next.is_some()
		{
			let next = next.unwrap();
			if next == self.requirement
			{
				*pos += self.requirement.len_utf8();
				Ok(&src[from..*pos])
			}
			else
			{
				Err(CharacterParserError::new(
					from,
					self.requirement,
					Some(next),
				))
			}
		}
		else
		{
			Err(CharacterParserError::new(from, self.requirement, None))
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		self.requirement
	}
}
