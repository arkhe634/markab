use crate::{
	Error,
	Parser,
};
use either::{
	Either,
	Left,
	Right,
};
use std::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
	},
	marker::PhantomData,
};

pub struct GenParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(&P::Output) -> Q,
	Q: Parser<'a, 'b>,
{
	requirement: P,
	generator: F,
	_a: PhantomData<&'a ()>,
	_b: PhantomData<&'b ()>,
}

impl<'a, 'b, P, F, Q> GenParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(&P::Output) -> Q,
	Q: Parser<'a, 'b>,
{
	pub fn new(requirement: P, generator: F) -> Self
	{
		Self {
			requirement,
			generator,
			_a: PhantomData,
			_b: PhantomData,
		}
	}
}

impl<'a, 'b, P, F, Q> Parser<'a, 'b> for GenParser<'a, 'b, P, F, Q>
where
	P: Parser<'a, 'b>,
	F: 'static + Fn(&P::Output) -> Q,
	Q: Parser<'a, 'b>,
{
	type Error = GenParserError<'a, 'b, P, Q>;
	type Output = (P::Output, Q::Output);
	type Requirement = GenParserRequirement<'a, 'b, P, Q>;
	type RequirementContext = Q;

	fn parse(&self, src: &'b str, pos: &mut usize) -> Result<Self::Output, Self::Error>
	{
		let from = *pos;
		let res1 = self
			.requirement
			.parse(src, pos)
			.map_err(|err| GenParserError::new(from, self.requirement(None), Left(err)))?;
		let parser = (self.generator)(&res1);
		let res2 = parser.parse(src, pos).map_err(|err| {
			GenParserError::new(from, self.requirement(Some(&parser)), Right(err))
		})?;
		Ok((res1, res2))
	}

	fn requirement(&self, context: Option<&Self::RequirementContext>) -> Self::Requirement
	{
		if let Some(context) = context
		{
			GenParserRequirement::new(
				self.requirement.requirement(None),
				Some(context.requirement(None)),
			)
		}
		else
		{
			GenParserRequirement::new(self.requirement.requirement(None), None)
		}
	}
}

pub struct GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	requirement: P::Requirement,
	generated: Option<Q::Requirement>,
}

impl<'a, 'b, P, Q> GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(requirement: P::Requirement, generated: Option<Q::Requirement>) -> Self
	{
		Self {
			requirement,
			generated,
		}
	}

	pub fn first(&self) -> &P::Requirement
	{
		&self.requirement
	}

	pub fn second(&self) -> Option<&Q::Requirement>
	{
		self.generated.as_ref()
	}
}

impl<'a, 'b, P, Q> Display for GenParserRequirement<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
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

pub struct GenParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	from: usize,
	requirement: GenParserRequirement<'a, 'b, P, Q>,
	cause: Either<P::Error, Q::Error>,
}

impl<'a, 'b, P, Q> GenParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
{
	pub fn new(
		from: usize,
		requirement: GenParserRequirement<'a, 'b, P, Q>,
		cause: Either<P::Error, Q::Error>,
	) -> Self
	{
		Self {
			from,
			requirement,
			cause,
		}
	}
}

impl<'a, 'b, P, Q> Error<'a, 'b> for GenParserError<'a, 'b, P, Q>
where
	P: Parser<'a, 'b>,
	Q: Parser<'a, 'b>,
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
			Right(_) => write!(f, "failed to parse {}", self.requirement.second().unwrap()),
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

	fn print(&self, f: &mut Formatter, depth: usize) -> FmtResult
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

// use crate::{
// 	Parser,
// 	Printer,
//};
// use std::{
// 	borrow::{
// 		Borrow,
// 		Cow,
// 	},
// 	fmt::{
// 		Formatter,
// 		Result as FmtResult,
// 	},
// 	marker::PhantomData,
//};
// pub struct GenParser<P, F, Q>
// where
// 	P: Parser,
// 	F: 'static + Fn(&P::Output) -> Q,
// 	Q: Parser,
//{
// 	requirement: P,
// 	generator: F,
//}
// impl<P, F, Q> GenParser<P, F, Q>
// where
// 	P: Parser,
// 	F: 'static + Fn(&P::Output) -> Q,
// 	Q: Parser,
//{
// 	pub fn new(requirement: P, generator: F) -> Self
// 	{
// 		Self {
// 			requirement,
// 			generator,
// 		}
// 	}
//
// 	pub fn requirement(&self) -> &P
// 	{
// 		&self.requirement
// 	}
//}
// impl<P, F, Q> Parser for GenParser<P, F, Q>
// where
// 	P: Parser,
// 	F: 'static + Fn(&P::Output) -> Q,
// 	Q: Parser,
//{
// 	type Error = GenParserError<Self::Requirement, Q>;
// 	type Output = (P::Output, Q::Output);
// 	type Requirement = GenParserRequirement<P, Q>;
//
// 	fn parse(&self, src: &str, pos: &mut usize) -> Result<Self::Output, Self::Error>
// 	{
// 		let from = *pos;
// 		let r0 = self
// 			.requirement
// 			.parse(src, pos)
// 			.map_err(|err| GenParserError::new(self.requirement(), None))?;
// 		let parser = (self.generator)(&r0);
// 		let r1 = parser
// 			.parse(src, pos)
// 			.map_err(|err| GenParserError::new(self.requirement(), Some(parser)))?;
// 		Ok((r0, r1))
// 	}
//}
// pub struct GenParserRequirement<P, Q>
// where
// 	P: Parser,
// 	Q: Parser,
//{
// 	requirement: P::Requirement,
// 	generated: Option<Q::Requirement>,
//}
// impl<P, Q> GenParserRequirement<P, Q>
// where
// 	P: Parser,
// 	Q: Parser,
//{
// 	pub fn new(requirement: P::Requirement, generated: Option<Q::Requirement>) -> Self
// 	{
// 		Self {
// 			requirement,
// 			generated,
// 		}
// 	}
//}
// impl<P, Q> Printer for GenParserRequirement<P, Q>
// where
// 	P: Parser,
// 	Q: Parser,
//{
// 	fn print(&self, f: &mut Formatter) -> FmtResult
// 	{
// 		write!(f, "gen")
// 	}
//}
// pub struct GenParserError<R, Q>
// where
// 	R: Printer,
// 	Q: Parser,
//{
// 	requirement: R,
// 	generated: Option<Q>,
//}
// impl<R, Q> GenParserError<R, Q>
// where
// 	R: Printer,
// 	Q: Parser,
//{
// 	pub fn new(requirement: R, generated: Option<Q>) -> Self
// 	{
// 		Self {
// 			requirement,
// 			generated,
// 		}
// 	}
//}
//
//// use crate::Parser;
//// use either::{
//// 	Either,
//// 	Left,
//// 	Right,
//// };
//// use std::{
//// 	borrow::{
//// 		Borrow,
//// 		Cow,
//// 	},
//// 	fmt::{
//// 		Display,
//// 		Formatter,
//// 		Result as FmtResult,
//// 	},
//// 	marker::PhantomData,
//// };
//// pub struct GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	requirement: P,
//// 	generator: F,
//// 	_a: PhantomData<& ()>,
//// }
//// impl<, P, F, Q> GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	pub fn new(requirement: P, generator: F) -> Self
//// 	{
//// 		Self {
//// 			requirement,
//// 			generator,
//// 			_a: PhantomData,
//// 		}
//// 	}
//// }
//// impl<, P, F, Q> Display for GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	fn fmt(&self, f: &mut Formatter) -> FmtResult
//// 	{
//// 		write!(f, "({}) -> gen", self.requirement)
//// 	}
//// }
//// impl<, P, F, Q> Parser<> for GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	type Error = GenParserError<, P, F, Q>;
//// 	type Output = (P::Output, Q::Output);
////
//// 	fn parse(&self, src: & str, pos: &mut usize) -> Result<Self::Output, Self::Error>
//// 	{
//// 		let from = *pos;
//// 		let res1 = self
//// 			.requirement
//// 			.parse(src, pos)
//// 			.map_err(|err| GenParserError::new(self.borrow(), Left(err)))?;
//// 		let parser = (self.generator)(&res1);
//// 		let res2 = parser
//// 			.parse(src, pos)
//// 			.map_err(|err| GenParserError::new(self.borrow(), Right(err)))?;
//// 		Ok((res1, res2))
//// 	}
//// }
//// pub struct GenParserError<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	requirement: Cow<, GenParser<, P, F, Q>>,
//// 	err: Either<P::Error, Q::Error>,
//// }
//// impl<, P, F, Q> GenParserError<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	pub fn new<T>(requirement: T, err: Either<P::Error, Q::Error>) -> Self
//// 	where
//// 		T: Borrow<GenParser<, P, F, Q>>,
//// 	{
//// 		Self {
//// 			requirement: Cow::new(requirement),
//// 			err,
//// 		}
//// 	}
//// }
//
//// use crate::Parser;
//// use either::{
//// 	Either,
//// 	Left,
//// 	Right,
//// };
//// use std::{
//// 	fmt::{
//// 		Display,
//// 		Formatter,
//// 		Result as FmtResult,
//// 	},
//// 	marker::PhantomData,
//// };
//// pub struct GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	requirement: P,
//// 	generator: F,
//// 	_a: PhantomData<& ()>,
//// }
//// impl<, P, F, Q> GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	pub fn new(requirement: P, generator: F) -> Self
//// 	{
//// 		Self {
//// 			requirement,
//// 			generator,
//// 			_a: PhantomData,
//// 		}
//// 	}
//// }
//// impl<, P, F, Q> Display for GenParser<, P, F, Q>
//// where
//// 	P: Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q: Parser<>,
//// {
//// 	fn fmt(&self, f: &mut Formatter) -> FmtResult
//// 	{
//// 		write!(f, "gen")
//// 	}
//// }
//// impl<, P, F, Q> Parser<> for GenParser<, P, F, Q>
//// where
//// 	P:  + Parser<>,
//// 	F: Fn(&P::Output) -> Q,
//// 	Q:  + Parser<>,
//// {
//// 	type Error = GenParserError<, P, Q>;
//// 	type Output = (P::Output, Q::Output);
////
//// 	fn parse(&self, src: & str, pos: &mut usize) -> Result<Self::Output, Self::Error>
//// 	{
//// 		let from = *pos;
//// 		let res1 = self
//// 			.requirement
//// 			.parse(src, pos)
//// 			.map_err(|err| GenParserError::new(from, &self.requirement, None, Left(err)))?;
//// 		let parser = (self.generator)(&res1);
//// 		let res2 = parser.parse(src, pos).map_err(|err| {
//// 			GenParserError::new(from, &self.requirement, Some(parser), Right(err))
//// 		})?;
//// 		Ok((res1, res2))
//// 	}
//// }
//// pub struct GenParserError<, P, Q>
//// where
//// 	P: Parser<>,
//// 	Q: Parser<>,
//// {
//// 	from: usize,
//// 	requirement: & P,
//// 	generated: Option<& Q>,
//// 	err: Either<P::Error, Q::Error>,
//// 	//_a: PhantomData<& ()>,
//// }
//// impl<, P, Q> GenParserError<, P, Q>
//// where
//// 	P: Parser<>,
//// 	Q: Parser<>,
//// {
//// 	pub fn new(
//// 		from: usize,
//// 		requirement: P,
//// 		generated: Option<Q>,
//// 		err: Either<P::Error, Q::Error>,
//// 	) -> Self
//// 	{
//// 		Self {
//// 			from,
//// 			requirement,
//// 			generated,
//// 			err,
//// 			//_a: PhantomData,
//// 		}
//// 	}
//// }
