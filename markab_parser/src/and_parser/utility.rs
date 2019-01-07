use crate::{
	and_parser::AndParser,
	Parser,
};

pub fn and<'a, P>(parser: P) -> AndParser<'a, P>
where
	P: Parser<'a>,
{
	AndParser::new(parser)
}
