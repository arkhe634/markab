pub trait Equal<T>: Sized
{
	fn ident(self) -> T;
}

impl<T> Equal<T> for T
{
	fn ident(self) -> T
	{
		self
	}
}
