use std::fmt::Display;

use crate::element::SemigroupElement;

pub mod algs;
pub mod impls;
pub mod word;

/// Trait to represent a semigroup, which is given as a list of generators.
///
/// This is a trait to allow for specific Semigroup types to implement special methods and validation.
pub trait Semigroup<U>: Display
where
    U: SemigroupElement,
{
    /// Return the identity if this semigroup has one
    fn id(&self) -> Option<U> {
        None
    }
    /// Return the generators of this Semigroup.
    fn generators(&self) -> &[U];
}
