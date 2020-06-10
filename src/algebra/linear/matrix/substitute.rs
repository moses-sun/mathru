pub trait Substitute<T>
{
    fn substitute_forward(self: &Self, b: T) -> T;

    fn substitute_backward(self: &Self, b: T) -> T;
}
