use std::fmt::{
	Formatter,
	Result as FmtResult,
};

pub trait Error
{
	fn from(&self, f: &mut Formatter) -> FmtResult;
	fn requirement(&self, f: &mut Formatter) -> FmtResult;
	fn result(&self, f: &mut Formatter) -> FmtResult;
	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult;

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
	{
		self.print_full(f, depth)
	}

	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult
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
}
