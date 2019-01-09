use crate::{
	map_parser::{
		MapParserError,
		MapParserRequirement,
	},
	Parser,
};
use std::fmt::{
	Debug,
	Formatter,
	Result as FmtResult,
};

pub struct MapParser<'a, P, R>
where
	P: 'a + Parser<'a>,
{
	requirement: P,
	mapper: &'a Fn(P::Output) -> R,
}

impl<'a, P, R> MapParser<'a, P, R>
where
	P: Parser<'a>,
{
	pub fn new(requirement: P, mapper: &'a Fn(P::Output) -> R) -> Self
	{
		Self {
			requirement,
			mapper,
		}
	}
}

impl<'a, P, R> Debug for MapParser<'a, P, R>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		f.debug_struct("MapParser")
			.field("requirement", &self.requirement)
			.field("mapper", &"..")
			.finish()
	}
}

impl<'a, P, R> Parser<'a> for MapParser<'a, P, R>
where
	P: Parser<'a>,
{
	type Error = MapParserError<'a, P>;
	type Output = R;
	type Requirement = MapParserRequirement<'a, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.parse(src, pos)
			.map(|res| (self.mapper)(res))
			.map_err(|err| MapParserError::new(from, self.requirement(None), err))
	}

	fn skip(&self, src: &'a str, pos: &mut usize) -> Result<(), Self::Error>
	{
		let from = *pos;
		self.requirement
			.skip(src, pos)
			.map_err(|err| MapParserError::new(from, self.requirement(None), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		MapParserRequirement::new(self.requirement.requirement(None))
	}
}

#[test]
fn test()
{
	use crate::string;
	let parser = string("test");
	let parser = parser.map(&|s| s.to_owned());
	let src = "test";
	let mut pos = 0;
	let res = parser.parse(src, &mut pos);
}
