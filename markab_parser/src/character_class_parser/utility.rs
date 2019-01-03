use crate::character_class_parser::CharacterClassParser;
use std::ops::Range;

pub fn character_class<'a>(
	not: bool,
	chars: &'a [char],
	ranges: &'a [Range<char>],
) -> CharacterClassParser<'a>
{
	CharacterClassParser::new(not, chars, ranges)
}
