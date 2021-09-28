use std::hash::Hash;

use super::alphabet::Alphabet;
use crate::element::SemigroupElement;

pub struct Word<'a, T, A>
where
    T: SemigroupElement,
    A: Hash,
{
    alphabet: &'a Alphabet<T, A>,
    word: Vec<T>,
}
