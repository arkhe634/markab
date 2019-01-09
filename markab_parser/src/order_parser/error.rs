use crate::{
	order_parser::OrderParserRequirement,
	Error,
	Parser,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct OrderParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	from: usize,
	requirement: OrderParserRequirement<'a, P1, P2>,
	cause: (P1::Error, P2::Error),
}

impl<'a, P1, P2> OrderParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	pub fn new(
		from: usize,
		requirement: OrderParserRequirement<'a, P1, P2>,
		cause: (P1::Error, P2::Error),
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, P1, P2> Error for OrderParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
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
		self.cause.0.print(f, depth)?;
		self.cause.1.print(f, depth)
	}
}

impl<'a, P1, P2> Display for OrderParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
