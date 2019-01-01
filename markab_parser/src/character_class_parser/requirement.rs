use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	ops::Range,
};

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
