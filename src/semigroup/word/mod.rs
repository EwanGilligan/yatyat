use std::{fmt::Display, hash::Hash, iter::FromIterator};

use crate::{element::SemigroupElement, semigroup::Semigroup, DetHashMap};

use snafu::Snafu;

pub struct Alphabet<T, A>
where
    A: Hash,
    T: SemigroupElement,
{
    // The identity of the semigroup if it is a monoid.
    identity: Option<T>,
    map: DetHashMap<A, T>,
}

impl<T, A> Alphabet<T, A>
where
    A: Hash + Ord + Clone + Display,
    T: SemigroupElement + Clone,
{
    pub fn new(
        semigroup: impl Semigroup<T>,
        symbol_iter: impl IntoIterator<Item = A>,
    ) -> Result<Self, AlphabetError<A>> {
        let mut map = DetHashMap::default();
        // Add a symbol for each generator, and add an association.
        for (gen, symbol) in semigroup.generators().iter().zip(symbol_iter) {
            // We don't allow duplicate symbols
            if map.contains_key(&symbol) {
                return Err(AlphabetError::DuplicateSymbol { symbol });
            }
            map.insert(symbol, gen.clone());
        }
        // Make sure the alphabet is large enough.
        if map.len() == semigroup.generators().len() {
            let identity = semigroup.id();
            Ok(Self { map, identity })
        } else {
            Err(AlphabetError::NotEnoughSymbols {})
        }
    }

    /// Return the generator that this symbol represents, or an error if the symbol is not in the alphabet.
    pub fn get_symbol(&self, symbol: &A) -> Result<&T, AlphabetError<A>> {
        match self.map.get(symbol) {
            Some(t) => Ok(t),
            None => Err(AlphabetError::MissingSymbol {
                symbol: symbol.clone(),
            }),
        }
    }

    /// Append to a word given a symbol in the alphabet
    pub fn append_word(&self, word: &Word<A>, symbol: &A) -> Result<Word<A>, AlphabetError<A>> {
        match self.map.get(symbol) {
            Some(_) => Ok(word.append(symbol)),
            None => Err(AlphabetError::MissingSymbol {
                symbol: symbol.clone(),
            }),
        }
    }

    /// Return the empty word for this alphabet.
    pub fn empty_word(&self) -> Word<A> {
        Word::empty_word()
    }

    /// Try to collapse a word to get the element that it represents.
    pub fn collapse_word(&self, word: Word<A>) -> Result<T, AlphabetError<A>> {
        // If we have the empty word, we try to return the identity if we have one.
        if word.is_empty_word() {
            match self.identity.as_ref() {
                Some(id) => Ok(id.clone()),
                None => Err(AlphabetError::NoIdentityElement {}),
            }
        } else {
            let init = self.get_symbol(&word.word[0])?;
            // Try to collapse the word, but we may encounter symbols we do not know.
            word.word[1..].iter().try_fold(init.clone(), |accum, a| {
                let element = self.get_symbol(a)?;
                Ok(accum.multiply(element))
            })
        }
    }
}

#[derive(Debug, Snafu)]
pub enum AlphabetError<A>
where
    A: Display,
{
    #[snafu(display("Symbol not in alphabet: {}", symbol))]
    MissingSymbol { symbol: A },
    #[snafu(display("Semigroup does not have an identity"))]
    NoIdentityElement {},
    #[snafu(display("Not enough symbols given"))]
    NotEnoughSymbols {},
    #[snafu(display("Symbol already in alphabet: {}", symbol))]
    DuplicateSymbol { symbol: A },
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

    fn is_empty_word(&self) -> bool {
        self.word.is_empty()
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
