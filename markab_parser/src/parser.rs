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

pub trait Parser<'a, 'b>: Debug
{
	type Error: Error;
	type Output;
	type Requirement: Debug + Display;
	type RequirementContext;

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>;

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		self.parse(src, pos).err()
	}

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

	fn zero_or_more(self) -> RepetitionParser<'a, 'b, Self>
	where
		Self: Sized,
	{
		RepetitionParser::new(self, 0, MAX)
	}

	fn one_or_more(self) -> RepetitionParser<'a, 'b, Self>
	where
		Self: Sized,
	{
		RepetitionParser::new(self, 1, MAX)
	}

	fn stringify(self) -> StringifyParser<'a, 'b, Self>
	where
		Self: Sized,
	{
		StringifyParser::new(self)
	}
}
