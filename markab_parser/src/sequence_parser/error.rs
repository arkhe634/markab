use crate::{
	sequence_parser::requirement::SequenceParserRequirement,
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
pub struct SequenceParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	from: usize,
	requirement: SequenceParserRequirement<'a, 'b, P, Q>,
	cause: Either<P::Error, Q::Error>,
}

impl<'a, 'b, P, Q> SequenceParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: SequenceParserRequirement<'a, 'b, P, Q>,
		cause: Either<P::Error, Q::Error>,
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P, Q> Error<'a, 'b> for SequenceParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
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
			Right(_) => write!(f, "failed to parse {}", self.requirement.second()),
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

impl<'a, 'b, P, Q> Display for SequenceParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
