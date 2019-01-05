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

impl<'a> Error for CharacterClassParserError<'a>
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
}

impl<'a> Display for CharacterClassParserError<'a>
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
