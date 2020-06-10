pub trait Function<Domain>
{
    type Codomain;
    fn eval(self: &Self, input: &Domain) -> Self::Codomain;
}
