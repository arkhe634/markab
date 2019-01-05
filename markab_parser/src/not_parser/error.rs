use crate::{
	not_parser::NotParserRequirement,
	Error,
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
pub struct NotParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	'a: 'b,
{
	from: usize,
	requirement: NotParserRequirement<'a, 'b, R>,
	cause: E,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R, E> NotParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	'a: 'b,
{
	pub fn new(from: usize, requirement: NotParserRequirement<'a, 'b, R>, cause: E) -> Self
	{
		Self {
			from,
			requirement,
			cause,
			_a: PhantomData,
			_b: PhantomData,
		}
	}

	pub fn cause(&self) -> &E
	{
		&self.cause
	}
}

impl<'a, 'b, R, E> Error for NotParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	'a: 'b,
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

impl<'a, 'b, R, E> Display for NotParserError<'a, 'b, R, E>
where
	R: Debug + Display,
	'a: 'b,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		self.print(f, 0)
	}
}
