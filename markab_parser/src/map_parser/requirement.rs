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
pub struct MapParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	requirement: R,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R> MapParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	pub fn new(requirement: R) -> Self
	{
		Self {
			requirement,
			_a: PhantomData,
			_b: PhantomData,
		}
	}

	pub fn requirement(&self) -> &R
	{
		&self.requirement
	}
}

impl<'a, 'b, R> Display for MapParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> mapped", self.requirement)
	}
}
