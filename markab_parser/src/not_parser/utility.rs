use crate::{
	not_parser::NotParser,
	Parser,
};

pub fn not<'a, P>(parser: P) -> NotParser<'a, P>
where
	P: Parser<'a>,
{
	NotParser::new(parser)
}
