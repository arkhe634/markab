use crate::{
	character_class_parser::CharacterClassParserRequirement,
	Error,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

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
