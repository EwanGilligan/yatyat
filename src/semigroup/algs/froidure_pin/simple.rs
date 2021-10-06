use std::{collections::HashSet, hash::Hash};

use crate::{
    element::SemigroupElement,
    semigroup::{
        word::{Alphabet, Word},
        Semigroup,
    },
    DetHashMap,
};

pub fn froidure_pin_simple<S, U>(
    semigroup: &S,
) -> (
    Vec<Word<usize>>,
    Vec<U>,
    DetHashMap<Word<usize>, Word<usize>>,
)
where
    S: Semigroup<U>,
    U: SemigroupElement + Eq + Hash,
{
    // TODO custom symbol iterator
    let symbol_iter = 0..semigroup.generators().len();
    // Create our alphabet for our words.
    let alphabet = Alphabet::new(semigroup, symbol_iter).unwrap();
    // Set of all symbols
    let symbols = alphabet.get_symbols();
    // List of words in miltary order
    let mut words = vec![alphabet.empty_word()];
    // Associate elements to words that represent them
    let mut elements: DetHashMap<U, usize> = DetHashMap::default();
    let mut relations: DetHashMap<Word<usize>, Word<usize>> = DetHashMap::default();
    // If we have an identity element, associate it with the empty word.
    if let Some(id) = semigroup.id() {
        elements.insert(id, 0);
    }
    let mut u = words[0].clone();
    loop {
        for gen in &symbols {
            let new_word = alphabet.append_word(&u, gen).unwrap();
            let collapse = alphabet.collapse_word(&new_word).unwrap();
            // If we find a new element
            if !elements.contains_key(&collapse) {
                words.push(new_word);
                elements.insert(collapse, words.len() - 1);
            } else {
                // TODO produce rule
                // We must have some word that is equal.
                let u_dash = elements.get(&collapse).unwrap();
                relations.insert(new_word, words[*u_dash].clone());
            }
        }
        let u_pos = words.iter().position(|w| w.eq(&u)).unwrap();
        if u_pos != words.len() - 1 {
            u = words[u_pos + 1].clone();
        } else {
            break;
        }
    }
    (words, elements.keys().cloned().collect(), relations)
}

mod tests {
    use std::time::{Duration, Instant};

    use crate::element::transformation::Transformation;
    use crate::semigroup::impls::transformation::TransformationSemigroup;

    use super::froidure_pin_simple;

    #[test]
    fn trivial_monoid() {
        // Trivial element for transformations of degree 3
        let s =
            TransformationSemigroup::new(&[Transformation::from_vec(3, vec![0, 1, 2]).unwrap()])
                .unwrap();
        let (words, elems, _relations) = froidure_pin_simple(&s);
        // empty word and id
        assert!(words.len() == 1);
        assert!(elems.len() == 1);
    }

    #[test]
    fn symmetric_group_5() {
        let s = TransformationSemigroup::new(&[
            Transformation::from_vec(5, vec![1, 0, 2, 3, 4]).unwrap(),
            Transformation::from_vec(5, vec![0, 2, 3, 4, 1]).unwrap(),
        ])
        .unwrap();
        let (words, elems, _relations) = froidure_pin_simple(&s);
        // empty word and id
        assert!(words.len() == 120);
        assert!(elems.len() == 120);
    }

    #[test]
    fn paper_example() {
        let s = TransformationSemigroup::new(&[
            Transformation::from_vec(6, vec![1, 1, 3, 3, 4, 5]).unwrap(),
            Transformation::from_vec(6, vec![4, 2, 3, 3, 5, 5]).unwrap(),
        ])
        .unwrap();
        let (words, elems, relations) = froidure_pin_simple(&s);
        for elem in elems.iter() {
            println!("{}", elem)
        }
        assert!(elems.len() == 7);
    }

    // #[test]
    // fn test() {
    //     let s = TransformationSemigroup::new(&[
    //         Transformation::from_vec(8, vec![1, 7, 2, 6, 0, 4, 1, 5]).unwrap(),
    //         Transformation::from_vec(8, vec![2, 4, 6, 1, 4, 5, 2, 7]).unwrap(),
    //         Transformation::from_vec(8, vec![3, 0, 7, 2, 4, 6, 2, 4]).unwrap(),
    //         Transformation::from_vec(8, vec![3, 2, 3, 4, 5, 3, 0, 1]).unwrap(),
    //         Transformation::from_vec(8, vec![4, 3, 7, 7, 4, 5, 0, 4]).unwrap(),
    //         Transformation::from_vec(8, vec![5, 6, 3, 0, 3, 0, 5, 1]).unwrap(),
    //         Transformation::from_vec(8, vec![6, 0, 1, 1, 1, 6, 3, 4]).unwrap(),
    //         Transformation::from_vec(8, vec![7, 7, 4, 0, 6, 4, 1, 7]).unwrap(),
    //     ])
    //     .unwrap();
    //     let start = Instant::now();
    //     let (words, elems) = froidure_pin_simple(&s);
    //     let duration = start.elapsed();
    //     println!("time={}s", duration.as_secs());
    //     dbg!(elems.len());
    // }
}
