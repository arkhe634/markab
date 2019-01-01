use crate::{
	character_class_parser::{
		CharacterClassParserError,
		CharacterClassParserRequirement,
	},
	Parser,
};
use std::ops::Range;

#[derive(Debug)]
pub struct CharacterClassParser<'a>
{
	not: bool,
	chars: &'a [char],
	ranges: &'a [Range<char>],
}

impl<'a> CharacterClassParser<'a>
{
	pub fn new(not: bool, chars: &'a [char], ranges: &'a [Range<char>]) -> Self
	{
		Self { not, chars, ranges }
	}
}

impl<'a, 'b> Parser<'a, 'b> for CharacterClassParser<'a>
{
	type Error = CharacterClassParserError<'a>;
	type Output = &'b str;
	type Requirement = CharacterClassParserRequirement<'a>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		if let Some(next) = src[from..].chars().next()
		{
			if self.not
			{
				for c in self.chars
				{
					if next == *c
					{
						return Err(CharacterClassParserError::new(
							from,
							self.requirement(None),
							Some(next),
						));
					}
				}
				for range in self.ranges
				{
					if range.start <= next && next <= range.end
					{
						return Err(CharacterClassParserError::new(
							from,
							self.requirement(None),
							Some(next),
						));
					}
				}
				*pos += next.len_utf8();
				Ok(&src[from..*pos])
			}
			else
			{
				for c in self.chars
				{
					if next == *c
					{
						*pos += next.len_utf8();
						return Ok(&src[from..*pos]);
					}
				}
				for range in self.ranges
				{
					if range.start <= next && next <= range.end
					{
						*pos += next.len_utf8();
						return Ok(&src[from..*pos]);
					}
				}
				Err(CharacterClassParserError::new(
					from,
					self.requirement(None),
					Some(next),
				))
			}
		}
		else
		{
			Err(CharacterClassParserError::new(
				from,
				self.requirement(None),
				None,
			))
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		CharacterClassParserRequirement::new(self.not, &self.chars, &self.ranges)
	}
}
