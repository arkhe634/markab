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
	usize::MAX,
};

#[derive(Debug)]
pub struct RepetitionParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P,
	min: usize,
	max: usize,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P> RepetitionParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P, min: usize, max: usize) -> Self
	{
		Self {
			requirement,
			min,
			max,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P> Parser<'a, 'b> for RepetitionParser<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	type Error = RepetitionParserError<'a, 'b, P>;
	type Output = Vec<P::Output>;
	type Requirement = RepetitionParserRequirement<'a, 'b, P>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let mut result = vec![];
		for i in 0..self.min
		{
			let res = self.requirement.parse(src, pos).map_err(|err| {
				*pos = from;
				RepetitionParserError::new(from, self.requirement(None), i, err)
			})?;
			result.push(res)
		}
		for _ in self.min..self.max
		{
			if let Ok(res) = self.requirement.parse(src, pos)
			{
				result.push(res)
			}
			else
			{
				break;
			}
		}
		Ok(result)
	}

	fn skip(&self, src: &'b str, pos: &mut usize) -> Option<Self::Error>
	{
		let from = *pos;
		for i in 0..self.min
		{
			if let Err(err) = self.requirement.parse(src, pos)
			{
				*pos = from;
				return Some(RepetitionParserError::new(
					from,
					self.requirement(None),
					i,
					err,
				));
			}
		}
		for _ in self.min..self.max
		{
			if let Err(_) = self.requirement.parse(src, pos)
			{
				break;
			}
		}
		None
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		RepetitionParserRequirement::new(self.requirement.requirement(None), self.min, self.max)
	}
}

#[derive(Debug)]
pub struct RepetitionParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	requirement: P::Requirement,
	min: usize,
	max: usize,
}

impl<'a, 'b, P> RepetitionParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement, min: usize, max: usize) -> Self
	{
		Self {
			requirement,
			min,
			max,
		}
	}
}

impl<'a, 'b, P> Display for RepetitionParserRequirement<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		match (self.min, self.max)
		{
			(0, 1) => write!(f, "{}?", self.requirement),
			(0, MAX) => write!(f, "{}*", self.requirement),
			(1, MAX) => write!(f, "{}+", self.requirement),
			(i, j) if i == j => write!(f, "{}{{{}}}", self.requirement, i),
			(i, j) => write!(f, "{}{{{},{}}}", self.requirement, i, j),
		}
	}
}

#[derive(Debug)]
pub struct RepetitionParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	from: usize,
	requirement: RepetitionParserRequirement<'a, 'b, P>,
	found: usize,
	cause: P::Error,
}

impl<'a, 'b, P> RepetitionParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: RepetitionParserRequirement<'a, 'b, P>,
		found: usize,
		cause: P::Error,
	) -> Self
	{
		Self {
			from,
			requirement,
			found,
			cause,
		}
	}
}

impl<'a, 'b, P> Error<'a, 'b> for RepetitionParserError<'a, 'b, P>
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
		write!(f, "succeed in parsing only {} time(s)", self.found)
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.cause.print(f, depth)
	}

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
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

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.print(f, depth)
	}
}

impl<'a, 'b, P> Display for RepetitionParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
