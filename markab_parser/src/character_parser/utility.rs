use crate::character_parser::parser::CharacterParser;

pub fn character(requirement: char) -> CharacterParser
{
	CharacterParser::new(requirement)
}
