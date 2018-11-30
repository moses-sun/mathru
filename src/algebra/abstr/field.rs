use algebra::abstr::Ring;
use std::ops::{Sub, SubAssign, Div, DivAssign, Neg};

pub trait Field : Ring + Sub<Self, Output = Self> + SubAssign<Self> + Div<Self, Output = Self> + DivAssign<Self>
{

}

