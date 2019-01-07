use crate::{
	order_parser::OrderParserRequirement,
	Error,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct OrderParserError<R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error,
	E2: Error,
{
	from: usize,
	requirement: OrderParserRequirement<R1, R2>,
	cause: (E1, E2),
}

impl<R1, R2, E1, E2> OrderParserError<R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error,
	E2: Error,
{
	pub fn new(from: usize, requirement: OrderParserRequirement<R1, R2>, cause: (E1, E2)) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<R1, R2, E1, E2> Error for OrderParserError<R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error,
	E2: Error,
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

impl<R1, R2, E1, E2> Display for OrderParserError<R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error,
	E2: Error,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
