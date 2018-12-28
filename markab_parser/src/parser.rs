use crate::{
	gen_parser::GenParser,
	map_parser::MapParser,
	order_parser::OrderParser,
	repetition_parser::RepetitionParser,
	sequence_parser::SequenceParser,
	Error,
};
use std::fmt::Display;

pub trait Parser<'a, 'b>
{
	type Error: Error<'a, 'b>;
	type Output: 'b;
	type Requirement: Display;
	type RequirementContext;

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn requirement(&self, context: Option<&Self::RequirementContext>) -> Self::Requirement;

	fn and_then<P>(self, next: P) -> SequenceParser<'a, 'b, Self, P>
	where
		Self: Sized,
		P: Parser<'a, 'b>,
	{
		SequenceParser::new(self, next)
	}

	fn or<P>(self, next: P) -> OrderParser<'a, 'b, Self, P>
	where
		Self: Sized,
		P: Parser<'a, 'b>,
	{
		OrderParser::new(self, next)
	}

	fn map<F, P>(self, mapper: F) -> MapParser<'a, 'b, Self, F, P>
	where
		Self: Sized,
		F: 'static + Fn(Self::Output) -> P,
		P: 'b,
	{
		MapParser::new(self, mapper)
	}

	fn and_gen<F, P>(self, generator: F) -> GenParser<'a, 'b, Self, F, P>
	where
		Self: Sized,
		F: 'static + Fn(&Self::Output) -> P,
		P: Parser<'a, 'b>,
	{
		GenParser::new(self, generator)
	}

	fn repeat(self, min: usize, max: usize) -> RepetitionParser<'a, 'b, Self>
	where
		Self: Sized,
	{
		RepetitionParser::new(self, min, max)
	}
}
