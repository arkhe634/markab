use crate::string_parser::StringParser;

pub fn string(requirement: &str) -> StringParser
{
	StringParser::new(requirement)
}
