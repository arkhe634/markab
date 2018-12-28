use std::fmt::{
	Formatter,
	Result as FmtResult,
};

pub trait Error<'a, 'b>
{
	fn from(&self, f: &mut Formatter) -> FmtResult;
	fn requirement(&self, f: &mut Formatter) -> FmtResult;
	fn result(&self, f: &mut Formatter) -> FmtResult;
	fn causes(&self, f: &mut Formatter, depth: usize) -> FmtResult;
	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult;
	fn print_full(&self, f: &mut Formatter, depth: usize) -> FmtResult;
}
