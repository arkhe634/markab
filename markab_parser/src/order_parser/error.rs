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
pub struct OrderParserError<'a, 'b, R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error<'a, 'b>,
	E2: Error<'a, 'b>,
{
	from: usize,
	requirement: OrderParserRequirement<'a, 'b, R1, R2>,
	cause: (E1, E2),
}

impl<'a, 'b, R1, R2, E1, E2> OrderParserError<'a, 'b, R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error<'a, 'b>,
	E2: Error<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: OrderParserRequirement<'a, 'b, R1, R2>,
		cause: (E1, E2),
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, R1, R2, E1, E2> Error<'a, 'b> for OrderParserError<'a, 'b, R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error<'a, 'b>,
	E2: Error<'a, 'b>,
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

impl<'a, 'b, R1, R2, E1, E2> Display for OrderParserError<'a, 'b, R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error<'a, 'b>,
	E2: Error<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}