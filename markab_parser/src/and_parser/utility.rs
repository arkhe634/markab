use crate::{
	and_parser::AndParser,
	Parser,
};

pub fn and<'a, 'b, P>(parser: P) -> AndParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	AndParser::new(parser)
}
