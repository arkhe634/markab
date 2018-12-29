use crate::{
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

pub struct CharacterParser
{
	requirement: char,
}

impl CharacterParser
{
	pub fn new(requirement: char) -> Self
	{
		Self { requirement }
	}
}

impl<'a, 'b> Parser<'a, 'b> for CharacterParser
{
	type Error = CharacterParserError;
	type Output = &'b str;
	type Requirement = char;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let next = src[from..].chars().next();
		if next.is_some()
		{
			let next = next.unwrap();
			if next == self.requirement
			{
				*pos += self.requirement.len_utf8();
				Ok(&src[from..*pos])
			}
			else
			{
				Err(CharacterParserError::new(
					from,
					self.requirement,
					Some(next),
				))
			}
		}
		else
		{
			Err(CharacterParserError::new(from, self.requirement, None))
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		self.requirement
	}
}

pub struct CharacterParserError
{
	from: usize,
	requirement: char,
	found: Option<char>,
}

impl CharacterParserError
{
	pub fn new(from: usize, requirement: char, found: Option<char>) -> Self
	{
		Self {
			from,
			requirement,
			found,
		}
	}
}

impl<'a, 'b> Error<'a, 'b> for CharacterParserError
{
	fn from(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{}", self.from)
	}

	fn requirement(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{:?}", self.requirement)
	}

	fn result(&self, f: &mut Formatter) -> FmtResult
	{
		match self.found
		{
			Some(found) => write!(f, "{:?} found", found),
			None => write!(f, "not found"),
		}
	}

	fn causes(&self, _: &mut Formatter, _: usize) -> FmtResult
	{
		Ok(())
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

impl Display for CharacterParserError
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}

pub fn character(requirement: char) -> CharacterParser
{
	CharacterParser::new(requirement)
}
