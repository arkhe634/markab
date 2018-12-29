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

#[derive(Debug)]
pub struct StringifyParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> StringifyParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P) -> Self
	{
		Self {
			requirement,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for StringifyParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	type Error = StringifyParserError<'a, 'b, P>;
	type Output = &'b str;
	type Requirement = StringifyParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		self.requirement
			.skip(src, pos)
			.map_or(Ok(&src[from..*pos]), |err| {
				Err(StringifyParserError::new(from, self.requirement(None), err))
			})
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		StringifyParserRequirement::new(self.requirement.requirement(None))
	}
}

#[derive(Debug)]
pub struct StringifyParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P::Requirement,
}

impl<'a, 'b, P> StringifyParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement) -> Self
	{
		Self { requirement }
	}
}

impl<'a, 'b, P> Display for StringifyParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> stringify", self.requirement)
	}
}

#[derive(Debug)]
pub struct StringifyParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	from: usize,
	requirement: StringifyParserRequirement<'a, 'b, P>,
	err: P::Error,
}

impl<'a, 'b, P> StringifyParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: StringifyParserRequirement<'a, 'b, P>,
		err: P::Error,
	) -> Self
	{
		Self {
			from,
			requirement,
			err,
		}
	}
}

impl<'a, 'b, P> Error<'a, 'b> for StringifyParserError<'a, 'b, P>
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
		self.err.print(f, depth)
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

impl<'a, 'b, P> Display for StringifyParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
