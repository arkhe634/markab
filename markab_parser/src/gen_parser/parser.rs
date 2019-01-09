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
use std::fmt::{
	Debug,
	Formatter,
	Result as FmtResult,
};

pub struct GenParser<'a, P1, P2>
where
	P1: 'a + Parser<'a>,
	P2: Parser<'a>,
{
	requirement: P1,
	generator: &'a Fn(&P1::Output) -> P2,
}

impl<'a, P1, P2> GenParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	pub fn new(requirement: P1, generator: &'a Fn(&P1::Output) -> P2) -> Self
	{
		Self {
			requirement,
			generator,
		}
	}
}

impl<'a, P1, P2> Debug for GenParser<'a, P1, P2>
where
	P1: Debug + Parser<'a>,
	P2: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		f.debug_struct("GenParser")
			.field("requirement", &self.requirement)
			.field("generator", &"..")
			.finish()
	}
}

impl<'a, P1, P2> Parser<'a> for GenParser<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	type Error = GenParserError<'a, P1, P2>;
	type Output = (P1::Output, P2::Output);
	type Requirement = GenParserRequirement<'a, P1, P2>;
	type RequirementContext = P2;

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

	fn skip(&self, src: &'a str, pos: &mut usize) -> Result<(), Self::Error>
	{
		let from = *pos;
		let res1 = match self.requirement.parse(src, pos)
		{
			Ok(ok) => ok,
			Err(err) => return Err(GenParserError::new(from, self.requirement(None), Left(err))),
		};
		let parser = (self.generator)(&res1);
		parser
			.skip(src, pos)
			.map_err(|err| GenParserError::new(from, self.requirement(None), Right(err)))
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
