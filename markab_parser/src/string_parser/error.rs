use crate::Error;
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct StringParserError<'a, 'b>
{
	from: usize,
	requirement: &'a str,
	src: &'b str,
}

impl<'a, 'b> StringParserError<'a, 'b>
{
	pub fn new(from: usize, requirement: &'a str, src: &'b str) -> Self
	{
		Self {
			from,
			requirement,
			src,
		}
	}
}

impl<'a, 'b> Error for StringParserError<'a, 'b>
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
		for p in (1..self.requirement.len() + 1).rev()
		{
			if self.src.is_char_boundary(p)
			{
				return write!(f, "{} found", &self.src[self.from..p]);
			}
		}
		write!(f, "not found")
	}

	fn causes(&self, _: &mut Formatter, _: usize) -> FmtResult
	{
		Ok(())
	}
}

impl<'a, 'b> Display for StringParserError<'a, 'b>
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
