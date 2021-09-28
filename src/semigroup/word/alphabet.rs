use std::hash::Hash;

use crate::{element::SemigroupElement, DetHashMap};

pub struct Alphabet<T, A>
where
    A: Hash,
    T: SemigroupElement,
{
    symbols: Vec<A>,
    symbol_iter: Box<dyn Iterator<Item = A>>,
    map: DetHashMap<A, T>,
}
