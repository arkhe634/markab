use crate::{
	gen_parser::GenParser,
	map_parser::MapParser,
	order_parser::OrderParser,
	repetition_parser::RepetitionParser,
	sequence_parser::SequenceParser,
	stringify_parser::StringifyParser,
	Error,
};
use std::{
	fmt::{
		Debug,
		Display,
	},
	usize::MAX,
};

pub trait Parser<'a>: Debug
{
	type Error: Error;
	type Output;
	type Requirement: Debug + Display;
	type RequirementContext;

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn skip(&self, src: &'a str, pos: &mut usize) -> Option<Self::Error>
	{
		self.parse(src, pos).err()
	}

	fn requirement(&self, context: Option<&Self::RequirementContext>) -> Self::Requirement;

	fn and_then<P>(self, next: P) -> SequenceParser<'a, Self, P>
	where
		Self: Sized,
		P: Parser<'a>,
	{
		SequenceParser::new(self, next)
	}

	fn or<P>(self, next: P) -> OrderParser<'a, Self, P>
	where
		Self: Sized,
		P: Parser<'a>,
	{
		OrderParser::new(self, next)
	}

	fn map<P>(self, mapper: &'a Fn(Self::Output) -> P) -> MapParser<'a, Self, P>
	where
		Self: Sized,
	{
		MapParser::new(self, mapper)
	}

	fn and_gen<P>(self, generator: &'a Fn(&Self::Output) -> P) -> GenParser<'a, Self, P>
	where
		Self: Sized,
		P: Parser<'a>,
	{
		GenParser::new(self, generator)
	}

	fn repeat(self, min: usize, max: usize) -> RepetitionParser<'a, Self>
	where
		Self: Sized,
	{
		RepetitionParser::new(self, min, max)
	}

	fn zero_or_more(self) -> RepetitionParser<'a, Self>
	where
		Self: Sized,
	{
		RepetitionParser::new(self, 0, MAX)
	}

	fn one_or_more(self) -> RepetitionParser<'a, Self>
	where
		Self: Sized,
	{
		RepetitionParser::new(self, 1, MAX)
	}

	fn stringify(self) -> StringifyParser<'a, Self>
	where
		Self: Sized,
	{
		StringifyParser::new(self)
	}
}
