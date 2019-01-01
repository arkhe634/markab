use crate::{
	not_parser::NotParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	from: usize,
	requirement: NotParserRequirement<'a, 'b, P>,
	cause: P::Output,
}

impl<'a, 'b, P> NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	pub fn new(from: usize, requirement: NotParserRequirement<'a, 'b, P>, cause: P::Output)
		-> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}

	pub fn cause(&self) -> &P::Output
	{
		&self.cause
	}
}

impl<'a, 'b, P> Error<'a, 'b> for NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
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
		write!(f, "success to parse")
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

impl<'a, 'b, P> Display for NotParserError<'a, 'b, P>
where
	P: Parser<'a, 'b>,
	'a: 'b,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
