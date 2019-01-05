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

impl<'a, 'b> Display for StringParserError<'a, 'b>
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
