use crate::{
	gen_parser::{
		GenParserError,
		GenParserRequirement,
	},
	Parser,
};
use either::{
	Left,
	Right,
};
use std::{
	fmt::{
		Debug,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct GenParser<'a, P, Q>
where
	P: 'a + Parser<'a>,
	Q: Parser<'a>,
{
	requirement: P,
	generator: &'a Fn(&P::Output) -> Q,
	_a: PhantomData<&'a ()>,
}

impl<'a, P, Q> GenParser<'a, P, Q>
where
	P: Parser<'a>,
	Q: Parser<'a>,
{
	pub fn new(requirement: P, generator: &'a Fn(&P::Output) -> Q) -> Self
	{
		Self {
			requirement,
			generator,
			_a: PhantomData,
		}
	}
}

impl<'a, P, Q> Debug for GenParser<'a, P, Q>
where
	P: Debug + Parser<'a>,
	Q: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		f.debug_struct("GenParser")
			.field("requirement", &self.requirement)
			.field("generator", &"..")
			.finish()
	}
}

impl<'a, P, Q> Parser<'a> for GenParser<'a, P, Q>
where
	P: Parser<'a>,
	Q: Parser<'a>,
{
	type Error = GenParserError<P::Requirement, Q::Requirement, P::Error, Q::Error>;
	type Output = (P::Output, Q::Output);
	type Requirement = GenParserRequirement<P::Requirement, Q::Requirement>;
	type RequirementContext = Q;

	fn parse(&self, src: &'a str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let res1 = self
			.requirement
			.parse(src, pos)
			.map_err(|err| GenParserError::new(from, self.requirement(None), Left(err)))?;
		let parser = (self.generator)(&res1);
		let res2 = parser.parse(src, pos).map_err(|err| {
			GenParserError::new(from, self.requirement(Some(&parser)), Right(err))
		})?;
		Ok((res1, res2))
	}

	fn skip(&self, src: &'a str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		let res1 = match self.requirement.parse(src, pos)
		{
			Ok(ok) => ok,
			Err(err) => return Some(GenParserError::new(from, self.requirement(None), Left(err))),
		};
		let parser = (self.generator)(&res1);
		parser
			.skip(src, pos)
			.map(|err| GenParserError::new(from, self.requirement(None), Right(err)))
	}

	fn requirement(&self, context: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		if let Some(context) = context
		{
			GenParserRequirement::new(
				self.requirement.requirement(None),
				Some(context.requirement(None)),
			)
		}
		else
		{
			GenParserRequirement::new(self.requirement.requirement(None), None)
		}
	}
}
