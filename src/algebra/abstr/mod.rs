mod number;
pub mod cast;
mod field;
mod ring;
mod semiring;


//pub mod identity;
mod natural;
mod integer;
mod real;
mod complex;

pub use self::semiring::{Semiring, Zero, One};
pub use self::ring::Ring;
pub use self::field::Field;
pub use self::number::Number;
pub use self::natural::Natural;
pub use self::integer::Integer;
pub use self::real::Real;
pub use self::complex::Complex;