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
pub struct AndParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	requirement: R,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R> AndParserRequirement<'a, 'b, R>
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
}

impl<'a, 'b, R> Display for AndParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "&{}", self.requirement)
	}
}
