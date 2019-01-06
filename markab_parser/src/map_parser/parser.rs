use crate::{
	map_parser::{
		MapParserError,
		MapParserRequirement,
	},
	Parser,
};
use std::{
	fmt::{
		Debug,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct MapParser<'a, P, F, Q>
where
	P: Parser<'a>,
	F: 'static + Fn(P::Output) -> Q,
{
	requirement: P,
	mapper: F,
	_a: PhantomData<&'a ()>,
}

impl<'a, P, F, Q> MapParser<'a, P, F, Q>
where
	P: Parser<'a>,
	F: 'static + Fn(P::Output) -> Q,
{
	pub fn new(requirement: P, mapper: F) -> Self
	{
		Self {
			requirement,
			mapper,
			_a: PhantomData,
		}
	}
}

impl<'a, P, F, Q> Debug for MapParser<'a, P, F, Q>
where
	P: Debug + Parser<'a>,
	F: 'static + Fn(P::Output) -> Q,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		f.debug_struct("MapParser")
			.field("requirement", &self.requirement)
			.field("mapper", &"..")
			.finish()
	}
}

impl<'a, P, F, Q> Parser<'a> for MapParser<'a, P, F, Q>
where
	P: Parser<'a>,
	F: 'static + Fn(P::Output) -> Q,
{
	type Error = MapParserError<P::Requirement, P::Error>;
	type Output = Q;
	type Requirement = MapParserRequirement<P::Requirement>;
	type RequirementContext = ();

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.parse(src, pos)
			.map(|res| (self.mapper)(res))
			.map_err(|err| MapParserError::new(from, self.requirement(None), err))
	}

	fn skip(&self, src: &'a str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		self.requirement
			.skip(src, pos)
			.map(|err| MapParserError::new(from, self.requirement(None), err))
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		MapParserRequirement::new(self.requirement.requirement(None))
	}
}
