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
pub struct StringifyParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	requirement: R,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R> StringifyParserRequirement<'a, 'b, R>
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

impl<'a, 'b, R> Display for StringifyParserRequirement<'a, 'b, R>
where
	R: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		write!(f, "({}) -> stringify", self.requirement)
	}
}
