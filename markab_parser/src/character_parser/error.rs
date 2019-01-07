use crate::Error;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
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

impl Error for CharacterParserError
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
}

impl Display for CharacterParserError
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
