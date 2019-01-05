use crate::{
	gen_parser::GenParserRequirement,
	Error,
};
use either::{
	Either,
	Left,
	Right,
};
use std::fmt::{
	Debug,
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct GenParserError<R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error,
	E2: Error,
{
	from: usize,
	requirement: GenParserRequirement<R1, R2>,
	cause: Either<E1, E2>,
}

impl<R1, R2, E1, E2> GenParserError<R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error,
	E2: Error,
{
	pub fn new(
		from: usize,
		requirement: GenParserRequirement<R1, R2>,
		cause: Either<E1, E2>,
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<R1, R2, E1, E2> Error for GenParserError<R1, R2, E1, E2>
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
		match &self.cause
		{
			Left(_) => write!(f, "failed to parse {}", self.requirement.first()),
			Right(_) => write!(f, "failed to parse {}", self.requirement.second().unwrap()),
		}
	}

	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		match &self.cause
		{
			Left(err) => err.print(f, depth),
			Right(err) => err.print(f, depth),
		}
	}
}

impl<R1, R2, E1, E2> Display for GenParserError<R1, R2, E1, E2>
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
