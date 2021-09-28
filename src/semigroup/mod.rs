use std::fmt::Display;

use crate::element::SemigroupElement;

pub mod transformation;

/// Trait to represent a semigroup, which is given as a list of generators.
///
/// This is a trait to allow for specific Semigroup types to implement special methods and validation.
pub trait Semigroup<U>: Display
where
    U: SemigroupElement,
{
    fn generators(&self) -> &[U];
}
