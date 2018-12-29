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
	ops::Range,
};

#[derive(Debug)]
pub struct CharacterClassParser<'a>
{
	not: bool,
	chars: &'a [char],
	ranges: &'a [Range<char>],
}

impl<'a> CharacterClassParser<'a>
{
	pub fn new(not: bool, chars: &'a [char], ranges: &'a [Range<char>]) -> Self
	{
		Self { not, chars, ranges }
	}
}

impl<'a, 'b> Parser<'a, 'b> for CharacterClassParser<'a>
{
	type Error = CharacterClassParserError<'a>;
	type Output = &'b str;
	type Requirement = CharacterClassParserRequirement<'a>;
	type RequirementContext = ();

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		if let Some(next) = src[from..].chars().next()
		{
			if self.not
			{
				for c in self.chars
				{
					if next == *c
					{
						return Err(CharacterClassParserError::new(
							from,
							self.requirement(None),
							Some(next),
						));
					}
				}
				for range in self.ranges
				{
					if range.start <= next && next <= range.end
					{
						return Err(CharacterClassParserError::new(
							from,
							self.requirement(None),
							Some(next),
						));
					}
				}
				*pos += next.len_utf8();
				Ok(&src[from..*pos])
			}
			else
			{
				for c in self.chars
				{
					if next == *c
					{
						*pos += next.len_utf8();
						return Ok(&src[from..*pos]);
					}
				}
				for range in self.ranges
				{
					if range.start <= next && next <= range.end
					{
						*pos += next.len_utf8();
						return Ok(&src[from..*pos]);
					}
				}
				Err(CharacterClassParserError::new(
					from,
					self.requirement(None),
					Some(next),
				))
			}
		}
		else
		{
			Err(CharacterClassParserError::new(
				from,
				self.requirement(None),
				None,
			))
		}
	}

	fn requirement(&self, _: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		CharacterClassParserRequirement::new(self.not, &self.chars, &self.ranges)
	}
}

#[derive(Debug)]
pub struct CharacterClassParserRequirement<'a>
{
	not: bool,
	chars: &'a [char],
	ranges: &'a [Range<char>],
}

impl<'a> CharacterClassParserRequirement<'a>
{
	pub fn new(not: bool, chars: &'a [char], ranges: &'a [Range<char>]) -> Self
	{
		Self { not, chars, ranges }
	}
}

impl<'a> Display for CharacterClassParserRequirement<'a>
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "[")?;
		if self.not
		{
			write!(f, "^")?;
		}
		for c in self.chars
		{
			write!(f, "{}", c)?;
		}
		for range in self.ranges
		{
			write!(f, "{}-{}", range.start, range.end)?;
		}
		Ok(())
	}
}

#[derive(Debug)]
pub struct CharacterClassParserError<'a>
{
	from: usize,
	requirement: CharacterClassParserRequirement<'a>,
	found: Option<char>,
}

impl<'a> CharacterClassParserError<'a>
{
	pub fn new(
		from: usize,
		requirement: CharacterClassParserRequirement<'a>,
		found: Option<char>,
	) -> Self
	{
		Self {
			from,
			requirement,
			found,
		}
	}
}

impl<'a, 'b> Error<'a, 'b> for CharacterClassParserError<'a>
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

impl<'a> Display for CharacterClassParserError<'a>
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}

pub fn character_class<'a>(
	not: bool,
	chars: &'a [char],
	ranges: &'a [Range<char>],
) -> CharacterClassParser<'a>
{
	CharacterClassParser::new(not, chars, ranges)
}
