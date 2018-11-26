mod number;
pub mod cast;

pub mod identity;
mod natural;
mod integer;
mod real;

pub use self::number::Number;
pub use self::natural::Natural;
pub use self::integer::Integer;
pub use self::real::Real;