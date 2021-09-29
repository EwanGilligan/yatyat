use std::{collections::HashSet, hash::Hash};

use crate::{
    element::SemigroupElement,
    semigroup::{word::Alphabet, Semigroup},
    DetHashSet,
};

pub fn froidure_pin_simple<S, U>(semigroup: &S)
where
    S: Semigroup<U>,
    U: SemigroupElement + Eq + Hash,
{
    // TODO custom symbol iterator
    let symbol_iter = 0..semigroup.generators().len();
    // Create our alphabet for our words.
    let alphabet = Alphabet::new(semigroup, symbol_iter).unwrap();
    let mut words = vec![alphabet.empty_word()];
    let mut elements: DetHashSet<U> = semigroup.generators().iter().cloned().collect();
    let mut u = words[0].clone();
    let symbols = alphabet.get_symbols();

    loop {
        for gen in &symbols {
            let new_word = alphabet.append_word(&u, gen).unwrap();
            let collapse = alphabet.collapse_word(&new_word).unwrap();
            // If we find a new element
            if elements.contains(&collapse) {
                words.push(new_word);
                elements.insert(collapse);
            } else {
                // Otherwise produce rule.
            }
        }
        if *words.last().unwrap() != u {
            let u_pos = words.iter().position(|w| w.eq(&u)).unwrap();
            u = words[u_pos + 1].clone();
        } else {
            break;
        }
    }
}
