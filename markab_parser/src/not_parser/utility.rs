use crate::{
	not_parser::NotParser,
	Parser,
};

pub fn not<'a, 'b, P>(parser: P) -> NotParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	NotParser::new(parser)
}
