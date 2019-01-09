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
pub struct NotParserError<'a, P>
where
	P: Parser<'a>,
{
	from: usize,
	requirement: NotParserRequirement<'a, P>,
	cause: P::Output,
}

impl<'a, P> NotParserError<'a, P>
where
	P: Parser<'a>,
{
	pub fn new(from: usize, requirement: NotParserRequirement<'a, P>, cause: P::Output) -> Self
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

impl<'a, P> Error for NotParserError<'a, P>
where
	P: Parser<'a>,
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
}

impl<'a, P> Display for NotParserError<'a, P>
where
	P: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
