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

pub struct MapParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(P::Output) -> Q,
{
	requirement: P,
	mapper: F,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, F, Q> MapParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(P::Output) -> Q,
{
	pub fn new(requirement: P, mapper: F) -> Self
	{
		Self {
			requirement,
			mapper,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P, F, Q> Debug for MapParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
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

impl<'a, 'b, P, F, Q> Parser<'a, 'b> for MapParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(P::Output) -> Q,
{
	type Error = MapParserError<'a, 'b, P>;
	type Output = Q;
	type Requirement = MapParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.parse(src, pos)
			.map(|res| (self.mapper)(res))
			.map_err(|err| MapParserError::new(from, self.requirement(None), err))
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
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
