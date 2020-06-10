pub trait Test<T>
{
    ///Test value
    fn value(self: &Self) -> T;

    /// Degree of freedom
    fn df(self: &Self) -> u32;

    ///
    fn p_value(self: &Self) -> T;
}
