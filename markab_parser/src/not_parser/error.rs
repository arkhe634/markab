use crate::{
	not_parser::NotParserRequirement,
	Error,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct NotParserError<R, E>
where
	R: Debug + Display,
{
	from: usize,
	requirement: NotParserRequirement<R>,
	cause: E,
}

impl<R, E> NotParserError<R, E>
where
	R: Debug + Display,
{
	pub fn new(from: usize, requirement: NotParserRequirement<R>, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}

	pub fn cause(&self) -> &E
	{
		&self.cause
	}
}

impl<R, E> Error for NotParserError<R, E>
where
	R: Debug + Display,
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

impl<R, E> Display for NotParserError<R, E>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
