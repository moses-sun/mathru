

pub struct OptimResult<T>
{
	arg: T
}



impl<T> OptimResult<T>
{
	pub fn new(arg: T) -> OptimResult<T>
	{
		OptimResult
		{
			arg: arg
		}
	}

	pub fn arg(self) -> T
	{
		return self.arg;
	}
}