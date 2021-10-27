use std::iter::{repeat, repeat_with};

use itertools::Itertools;

use super::{CayleyGraphType, FroidurePinBuilder, FroidurePinResult};
use crate::{
    element::SemigroupElement,
    semigroup::{word::Word, Semigroup},
    utils::vec2::Vec2,
    DetHashMap,
};

#[derive(Debug)]
pub struct FroidurePin<T>
where
    T: SemigroupElement + std::hash::Hash,
{
    // Current length of the word we're considering
    current_word_length: usize,
    // Original generators of the semigroup
    generators: Vec<T>,
    // Elements sorted in military order
    elements: Vec<T>,
    // Map of elements to position in gens
    element_map: DetHashMap<T, usize>,
    // Rewrite rules that index into the elements
    rewrite_rules: Vec<(Word<usize>, Word<usize>)>,
    // The left and right Cayley graphs, which index into the elements.
    left_cayley_graph: CayleyGraphType,
    right_cayley_graph: CayleyGraphType,
    // Store if a given element is reduced, i.e if it was new when we first encountered it
    reduced: Vec2<bool>,
    // Various bits of bookkeeping, which can be summarised by
    // elements[i] = prefix[i] * last[i] = first[i] * suffix[i]
    // At index i store the index of what we multiplied on the left by to get the value at index i in the elements
    prefix: Vec<Option<usize>>,
    // At index i store the index of the generator we multiplied on the right by to get the value at index i in the elements
    last: Vec<usize>,
    // At index i store the index of what we multiplied on the right by to get the value at index i in the elements
    suffix: Vec<Option<usize>>,
    // At index i store the index of the generator we multiplied by to get the value at index i in the elements
    first: Vec<usize>,
    // Index i stores the length of the word representing elements[i]
    length: Vec<usize>,
}

impl<T> FroidurePin<T>
where
    T: SemigroupElement + std::hash::Hash + std::fmt::Debug,
{
    fn new<U>(gens: &U) -> Self
    where
        U: Semigroup<T>,
    {
        // Filter out duplicate generators and the identity
        let generators: Vec<T> = gens
            .generators()
            .iter()
            .unique()
            .filter(|s| !s.is_id())
            .cloned()
            .collect();
        // Initial elements are just the generators
        let mut elements = generators.clone();
        // Insert identity into position zero.
        elements.insert(0, gens.id().unwrap());
        let mut element_map = DetHashMap::default();
        let rewrite_rules = Vec::new();
        // Vecs for info about each element
        let mut prefix = Vec::new();
        let mut last = Vec::new();
        let mut suffix = Vec::new();
        let mut first = Vec::new();
        let mut length = Vec::new();
        // Initialise identity
        prefix.push(None);
        last.push(0);
        suffix.push(None);
        first.push(0);
        length.push(0);
        element_map.insert(elements[0].clone(), 0);
        // Now initialise the above
        for index in 1..elements.len() {
            element_map.insert(elements[index].clone(), index);
            // Prefix of a generator is the empty word
            prefix.push(Some(0));
            last.push(index);
            // Suffix is similarly the empty word
            suffix.push(Some(0));
            first.push(index);
            length.push(1);
        }
        // 2d arrays for the Cayley graphs and if a word is reduced.
        let mut left_cayley_graph = Vec2::new(elements.len(), elements.len());
        let mut right_cayley_graph = Vec2::new(elements.len(), elements.len());
        // Initialise identity for the graphs
        for i in 1..=generators.len() {
            left_cayley_graph[(i, 0)] = Some(i);
            left_cayley_graph[(0, i)] = Some(i);
            right_cayley_graph[(i, 0)] = Some(i);
            right_cayley_graph[(0, i)] = Some(i);
        }
        let reduced = Vec2::new(elements.len(), elements.len());
        // Other information
        let current_word_length = 1;
        Self {
            generators,
            elements,
            element_map,
            rewrite_rules,
            reduced,
            prefix,
            last,
            suffix,
            first,
            length,
            left_cayley_graph,
            right_cayley_graph,
            current_word_length,
        }
    }

    /// Given u and x, find v such that ux = v
    fn get_right_cayley_element(&self, element: usize, generator_index: usize) -> Option<usize> {
        debug_assert!(generator_index != 0 && generator_index <= self.generators.len());
        self.right_cayley_graph[(element, generator_index)]
    }

    /// Given u and x, find v such that xu = v
    fn get_left_cayley_element(&self, element: usize, generator_index: usize) -> Option<usize> {
        debug_assert!(generator_index != 0 && generator_index <= self.generators.len());
        self.left_cayley_graph[(element, generator_index)]
    }

    // Convert an index into a word of the generators
    fn pos_to_word(&self, pos: usize) -> Word<usize> {
        let mut cur_pos = pos;
        // We repeatedly take the last value to create our word
        repeat_with(move || {
            let first = self.first[cur_pos];
            cur_pos = self.suffix[cur_pos].unwrap();
            first
        })
        .take(self.length[pos])
        .collect()
    }

    fn run(&mut self) {
        // First multiply all generators by themselves
        for i in 1..=self.generators.len() {
            for j in 1..=self.generators.len() {
                let product = self.elements[i].multiply(&self.elements[j]);
                match self.element_map.get(&product) {
                    Some(&index) => {
                        // Add rule
                        let rhs = self.pos_to_word(index);
                        let lhs = self.pos_to_word(i).append(&j);
                        self.rewrite_rules.push((lhs, rhs));
                        // Update cayley graphs
                        self.right_cayley_graph[(i, j)] = Some(index);
                        self.left_cayley_graph[(j, i)] = Some(index);
                    }

                    None => {
                        // Add the new element
                        let new_pos = self.elements.len();
                        self.elements.push(product.clone());
                        self.element_map.insert(product, new_pos);
                        // Then update first, last, suffix, and prefix
                        self.first.push(i);
                        self.last.push(j);
                        self.prefix.push(Some(i));
                        self.suffix.push(Some(j));
                        // Update reduced table
                        self.reduced.add_row();
                        self.reduced[(i, j)] = true;
                        // Update right cayley graph, left cayley graph will be done later
                        self.right_cayley_graph.add_row();
                        self.left_cayley_graph.add_row();
                        // a_i * a_j = new element
                        self.right_cayley_graph[(i, j)] = Some(new_pos);
                        self.left_cayley_graph[(j, i)] = Some(new_pos);
                        debug_assert!(self.elements.len() == self.element_map.len());
                        debug_assert!(self.elements.len() == self.first.len());
                        debug_assert!(self.elements.len() == self.last.len());
                        debug_assert!(self.elements.len() == self.prefix.len());
                        debug_assert!(self.elements.len() == self.suffix.len());
                        // Update length, this is simply one more than u
                        self.length.push(self.length[i] + 1);
                    }
                }
            }
        }
        // Then continue unless we found no new elements
        if self.generators.len() + 1 == self.elements.len() {
            return;
        }
        self.current_word_length = 2;
        // Take first non generator element
        let mut u = self.generators.len() + 1;
        let mut v = u;
        loop {
            // Computation of u*a_i
            while u < self.elements.len() && self.length[u] == self.current_word_length {
                let first = self.first[u];
                let suffix = self.suffix[u].expect("Should be larger than 2");
                // Iterate over the generators to consider products of the form sa_i
                for i in 1..=self.generators.len() {
                    // If sa_i is not reduced
                    if !self.reduced[(suffix, i)] {
                        // We get s*a_i from the right cayley graph.
                        let suffix_gen = self
                            .get_right_cayley_element(suffix, i)
                            .expect("Should be present");
                        match suffix_gen {
                            // Identity
                            0 => {
                                debug_assert!(self.right_cayley_graph[(u, i)] == None);
                                self.right_cayley_graph[(u, i)] = Some(first);
                            }
                            // Non identity
                            _ => {
                                let last = self.last[suffix_gen];
                                let prefix = self.prefix[suffix_gen].expect("Should be present");
                                let first_prefix = self
                                    .get_left_cayley_element(prefix, first)
                                    .expect("Should be present");
                                let first_prefix_last = self
                                    .get_right_cayley_element(first_prefix, last)
                                    .expect("Should be present");
                                debug_assert!(self.right_cayley_graph[(u, i)] == None);
                                self.right_cayley_graph[(u, i)] = Some(first_prefix_last);
                            }
                        }
                    } else {
                        let product = self.elements[u].multiply(&self.elements[i]);
                        match self.element_map.get(&product) {
                            // If we have already seen this element, add a new rule
                            Some(&index) => {
                                // Add rule
                                let rhs = self.pos_to_word(index);
                                let lhs = self.pos_to_word(u).append(&i);
                                self.rewrite_rules.push((lhs, rhs));
                                // Update right cayley graph
                                self.right_cayley_graph[(u, i)] = Some(index)
                            }
                            // Otherwise we have a new element, so we add to the collection.
                            None => {
                                // Add the new element
                                let new_pos = self.elements.len();
                                self.elements.push(product.clone());
                                self.element_map.insert(product, new_pos);
                                // Then update first, last, suffix, and prefix
                                self.first.push(first);
                                self.last.push(i);
                                self.prefix.push(Some(u));
                                let suffix = {
                                    let u_suffix = self.suffix[u].expect("Should be present.");
                                    self.get_right_cayley_element(u_suffix, i)
                                        .expect("Should already be present")
                                };
                                self.suffix.push(Some(suffix));
                                // Update reduced table
                                self.reduced.add_row();
                                self.reduced[(u, i)] = true;
                                // Update right cayley graph, left cayley graph will be done later
                                self.right_cayley_graph.add_row();
                                self.left_cayley_graph.add_row();
                                // u * a_i = new element
                                debug_assert!(self.right_cayley_graph[(u, i)] == None);
                                self.right_cayley_graph[(u, i)] = Some(new_pos);
                                // Update length, this is simply one more than u
                                self.length.push(self.length[u] + 1);
                            }
                        }
                    }
                }
                // Go to the next element
                u += 1;
            }
            u = v;
            // Now compute a_i * u to fill in left cayley graph
            while u < self.elements.len() && self.length[u] == self.current_word_length {
                let prefix = self.prefix[u].expect("Should be present.");
                let last = self.last[u];
                for i in 1..=self.generators.len() {
                    // We work out what a_i * u is from prior information.
                    let res = {
                        let ap = self
                            .get_left_cayley_element(prefix, i)
                            .expect("Should already be computed");
                        self.get_right_cayley_element(ap, last)
                            .expect("Should already be computed.")
                    };
                    // a_i * u = res
                    self.left_cayley_graph[(u, i)] = Some(res);
                }
                // Go onto the next element.
                u += 1;
            }
            v = u;
            self.current_word_length += 1;
            // Break if we've reached the last possible u
            if u == self.elements.len() {
                break;
            }
        }
    }
}

impl<T> FroidurePinBuilder<T> for FroidurePin<T>
where
    T: SemigroupElement + std::hash::Hash + std::fmt::Debug,
{
    fn new<U>(semigroup: &U) -> Self
    where
        U: Semigroup<T>,
    {
        FroidurePin::new(semigroup)
    }

    fn build(mut self) -> FroidurePinResult<T> {
        // Run Froidure-Pin
        self.run();
        FroidurePinResult {
            generators: self.generators,
            elements: self.elements,
            element_map: self.element_map,
            rewrite_rules: self.rewrite_rules,
            left_cayley_graph: self.left_cayley_graph,
            right_cayley_graph: self.right_cayley_graph,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        element::transformation::Transformation,
        semigroup::{
            algs::froidure_pin::{
                froidure_pin_impl::FroidurePin, simple::FroidurePinSimple, FroidurePinBuilder,
            },
            impls::transformation::TransformationSemigroup,
        },
    };

    #[test]
    fn transformation_monoid_5() {
        let s = TransformationSemigroup::new(&[
            Transformation::from_vec(5, vec![1, 0, 2, 3, 4]).unwrap(),
            Transformation::from_vec(5, vec![1, 2, 3, 4, 0]).unwrap(),
            Transformation::from_vec(5, vec![1, 1, 2, 3, 4]).unwrap(),
        ])
        .unwrap();
        let fp = FroidurePin::new(&s);
        let res = fp.build();
        dbg!(&res.elements.len());
        assert!(res.elements.len() == 3125);
    }
}
