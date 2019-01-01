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
pub struct GenParserRequirement<'a, 'b, R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	requirement: R1,
	generated: Option<R2>,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, R1, R2> GenParserRequirement<'a, 'b, R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	pub fn new(requirement: R1, generated: Option<R2>) -> Self
	{
		Self {
			requirement,
			generated,
			_a: PhantomData,
			_b: PhantomData,
		}
	}

	pub fn first(&self) -> &R1
	{
		&self.requirement
	}

	pub fn second(&self) -> Option<&R2>
	{
		self.generated.as_ref()
	}
}

impl<'a, 'b, R1, R2> Display for GenParserRequirement<'a, 'b, R1, R2>
where
	R1: Debug + Display,
	R2: Debug + Display,
{
	fn fmt(&self, f: &mut Formatter) -> FmtResult
	{
		if let Some(generated) = &self.generated
		{
			write!(f, "{} {}", self.requirement, generated)
		}
		else
		{
			write!(f, "({}) -> gen", self.requirement)
		}
	}
}
