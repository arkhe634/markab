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
	E1: Error,
	E2: Error,
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
	E1: Error,
	E2: Error,
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

impl<'a, 'b, R1, R2, E1, E2> Error for SequenceParserError<'a, 'b, R1, R2, E1, E2>
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
}

impl<'a, 'b, R1, R2, E1, E2> Display for SequenceParserError<'a, 'b, R1, R2, E1, E2>
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
