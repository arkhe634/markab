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
pub struct NotParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
	'a: 'b,
{
	requirement: R,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R> NotParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
	'a: 'b,
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

impl<'a, 'b, R> Display for NotParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
	'a: 'b,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "!{}", self.requirement)
	}
}
