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
pub struct OrderParserRequirement<'a, 'b, R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	first: R1,
	second: R2,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R1, R2> OrderParserRequirement<'a, 'b, R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	pub fn new(first: R1, second: R2) -> Self
	{
		Self {
			first,
			second,
			_a: PhantomData,
			_b: PhantomData,
		}
	}

	pub fn first(&self) -> &R1
	{
		&self.first
	}

	pub fn second(&self) -> &R2
	{
		&self.second
	}
}

impl<'a, 'b, R1, R2> Display for OrderParserRequirement<'a, 'b, R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "{} {}", self.first, self.second)
	}
}
