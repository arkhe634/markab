use crate::{
	gen_parser::GenParserRequirement,
	Error,
	Parser,
};
use either::{
	Either,
	Left,
	Right,
};
use std::fmt::{
	Display,
	Formatter,
	Result as FmtResult,
};

#[derive(Debug)]
pub struct GenParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	from: usize,
	requirement: GenParserRequirement<'a, P1, P2>,
	cause: Either<P1::Error, P2::Error>,
}

impl<'a, P1, P2> GenParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	pub fn new(
		from: usize,
		requirement: GenParserRequirement<'a, P1, P2>,
		cause: Either<P1::Error, P2::Error>,
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, P1, P2> Error for GenParserError<'a, P1, P2>
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

impl<'a, P1, P2> Display for GenParserError<'a, P1, P2>
where
	P1: Parser<'a>,
	P2: Parser<'a>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
