use crate::{
	sequence_parser::requirement::SequenceParserRequirement,
	Error,
};
use either::{
	Either,
	Left,
	Right,
};
use std::{
	fmt::{
		Debug,
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

#[derive(Debug)]
pub struct SequenceParserError<'a, 'b, R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error<'a, 'b>,
	E2: Error<'a, 'b>,
{
	from: usize,
	requirement: SequenceParserRequirement<'a, 'b, R1, R2>,
	cause: Either<E1, E2>,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R1, R2, E1, E2> SequenceParserError<'a, 'b, R1, R2, E1, E2>
where
	R1: Debug + Display,
	R2: Debug + Display,
	E1: Error<'a, 'b>,
	E2: Error<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: SequenceParserRequirement<'a, 'b, R1, R2>,
		cause: Either<E1, E2>,
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, R1, R2, E1, E2> Error<'a, 'b> for SequenceParserError<'a, 'b, R1, R2, E1, E2>
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

impl<'a, 'b, R1, R2, E1, E2> Display for SequenceParserError<'a, 'b, R1, R2, E1, E2>
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
