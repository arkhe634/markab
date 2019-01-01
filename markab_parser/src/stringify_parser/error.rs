use crate::{
	stringify_parser::StringifyParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

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
