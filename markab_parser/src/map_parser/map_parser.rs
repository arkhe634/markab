use crate::{
	Error,
	Parser,
};
use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct MapParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(P::Output) -> Q,
	Q: 'b,
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
	Q: 'b,
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

impl<'a, 'b, P, F, Q> Parser<'a, 'b> for MapParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(P::Output) -> Q,
	Q: 'b,
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

pub struct MapParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> MapParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}

	pub fn requirement(&self) -> &P::Requirement
	{
		&self.requirement
	}
}

impl<'a, 'b, P> Display for MapParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> mapped", self.requirement)
	}
}

pub struct MapParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	from: usize,
	requirement: MapParserRequirement<'a, 'b, P>,
	cause: P::Error,
}

impl<'a, 'b, P> MapParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(from: usize, requirement: MapParserRequirement<'a, 'b, P>, cause: P::Error) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P> Error<'a, 'b> for MapParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn from(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{}", self.from)
	}

	fn requirement(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{}", self.requirement)
	}

	fn result(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "failed to parse")
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.cause.print(f, depth)
	}

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.causes(f, depth)
	}

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		for _ in 0..depth
		{
			write!(f, "\t")?;
		}
		write!(f, "at position ")?;
		self.from(f)?;
		write!(f, " required ")?;
		self.requirement(f)?;
		write!(f, " but ")?;
		self.result(f)?;
		write!(f, ".\n")?;
		self.causes(f, depth + 1)
	}
}
