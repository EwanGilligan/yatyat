use std::{fmt::Display, hash::Hash, iter::FromIterator};

use crate::{
    element::SemigroupElement,
    semigroup::{self, Semigroup},
    DetHashMap, DetHashSet,
};

pub struct Alphabet<T, A>
where
    A: Hash,
    T: SemigroupElement,
{
    map: DetHashMap<A, T>,
}

impl<T, A> Alphabet<T, A>
where
    A: Hash + Ord + Clone,
    T: SemigroupElement + Clone,
{
    pub fn new(semigroup: impl Semigroup<T>, symbol_iter: impl IntoIterator<Item = A>) -> Self {
        let mut map = DetHashMap::default();
        // Add a symbol for each generator, and add an association.
        for (gen, symbol) in semigroup.generators().iter().zip(symbol_iter) {
            debug_assert!(!map.contains_key(&symbol));
            map.insert(symbol, gen.clone());
        }
        // TODO add proper error handling.
        debug_assert!(map.len() == semigroup.generators().len());
        Self { map }
    }

    /// Append to a word given a symbol in the alphabet
    pub fn append_word(&self, word: &Word<A>, symbol: &A) -> Word<A> {
        // TODOD error handling
        assert!(self.map.contains_key(symbol));
        word.append(symbol)
    }

    /// Return the empty word for this alphabet.
    pub fn empty_word(&self) -> Word<A> {
        Word::empty_word()
    }
}

/// Struct that represents a word from an alphabet.
/// This should be used in the context of an Alphabet, to provide sanity checking.
#[derive(Debug, Clone)]
pub struct Word<A>
where
    A: Clone,
{
    word: Vec<A>,
}

impl<A> Word<A>
where
    A: Ord + Clone,
{
    /// Create the empty word.
    fn empty_word() -> Self {
        Self {
            word: Vec::with_capacity(0),
        }
    }

    /// Append to a word, giving a new word
    fn append(&self, a: &A) -> Self {
        self.word
            .iter()
            .cloned()
            .chain(std::iter::once(a.clone()))
            .collect()
    }
}

impl<A> FromIterator<A> for Word<A>
where
    A: Ord + Clone,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Word {
            word: iter.into_iter().collect(),
        }
    }
}

impl<A> Display for Word<A>
where
    A: Display + Clone + Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for symbol in &self.word {
            write!(f, "{}", symbol)?;
        }
        Ok(())
    }
}
