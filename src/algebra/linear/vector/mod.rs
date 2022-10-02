#[macro_use]
pub mod vector_impl;
pub mod vectorintoiterator;
pub mod vectoriterator;
pub mod vectoriteratormut;

mod add;
mod add_assign;
mod div;
mod index;
mod mul;
mod mul_assign;
mod sub;
mod sub_assign;

#[cfg(feature = "convert-mint")]
mod mint;

pub use self::{
    vector_impl::Vector, vectorintoiterator::VectorIntoIterator, vectoriterator::VectorIterator,
    vectoriteratormut::VectorIteratorMut,
};
