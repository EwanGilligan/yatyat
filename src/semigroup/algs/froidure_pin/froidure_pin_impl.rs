use itertools::Itertools;

use super::{CayleyGraphType, FroidurePinBuilder, FroidurePinResult};
use crate::{
    element::{self, SemigroupElement},
    semigroup::{word::Word, Semigroup},
    utils::vec2::Vec2,
    DetHashMap,
};

struct FroidurePin<T>
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
    T: SemigroupElement + std::hash::Hash,
{
    fn new(gens: &[T]) -> Self {
        // Filter out duplicate generators.
        let generators: Vec<T> = gens.iter().unique().cloned().collect();
        // Initial elements are just the generators
        let elements = generators.clone();
        let mut element_map = DetHashMap::default();
        let rewrite_rules = Vec::new();
        // Vecs for info about each element
        let mut prefix = Vec::new();
        let mut last = Vec::new();
        let mut suffix = Vec::new();
        let mut first = Vec::new();
        let mut length = Vec::new();
        // Now initialise the above
        for (index, element) in elements.iter().enumerate() {
            element_map.insert(element.clone(), index);
            // Prefix of a generator is the empty word
            prefix.push(None);
            last.push(index);
            // Suffix is similarly the empty word
            suffix.push(None);
            first.push(index);
            length.push(1);
        }
        // 2d arrays for the Cayley graphs and if a word is reduced.
        let left_cayley_graph = Vec2::new(elements.len(), elements.len());
        let right_cayley_graph = Vec2::new(elements.len(), elements.len());
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
        debug_assert!(generator_index < self.generators.len());
        self.right_cayley_graph
            .get_row(element)
            .iter()
            .position(|x_opt| x_opt.as_ref().map_or(false, |x| *x == generator_index))
    }

    /// Given u and x, find v such that xu = v
    fn get_left_cayley_element(&self, element: usize, generator_index: usize) -> Option<usize> {
        debug_assert!(generator_index < self.generators.len());
        self.left_cayley_graph
            .get_row(element)
            .iter()
            .position(|x_opt| x_opt.as_ref().map_or(false, |x| *x == generator_index))
    }

    fn run(&mut self) {
        // First multiply all generators by themselves
        for i in 0..self.generators.len() {
            for j in 0..self.generators.len() {
                let product = self.generators[i].multiply(&self.generators[j]);
                if self.element_map.contains_key(&product) {
                    // TODO add as a rule
                } else {
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
                    self.reduced.add_col();
                    self.reduced[(i, j)] = true;
                    // Update right cayley graph, left cayley graph will be done later
                    self.right_cayley_graph.add_row();
                    self.right_cayley_graph.add_col();
                    self.left_cayley_graph.add_row();
                    self.left_cayley_graph.add_col();
                    // a_i * a_j = new element
                    self.right_cayley_graph[(i, new_pos)] = Some(j);
                    self.left_cayley_graph[(j, new_pos)] = Some(i);
                    // TODO remove
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
        // Then continue unless we found no new elements
        if self.generators.len() == self.elements.len() {
            return;
        }
        self.current_word_length = 2;
        let mut u = 0;
        let mut v = 0;
        let mut last = self.generators.len() - 1;
        loop {
            // Computation of u*a_i
            while self.length[u] == self.current_word_length {
                let first = self.first[u];
                let suffix = self.suffix[u].expect("Should be larger than 2");
                // Iterate over the generators to consider products of the form sa_i
                for i in 0..self.generators.len() {
                    // If sa_i is not reduced
                    if !self.reduced[(suffix, i)] {
                        // We get s*a_i from the right cayley graph.
                        let suffix_gen = self.get_right_cayley_element(suffix, i);
                        match suffix_gen {
                            Some(suffix_gen) => {
                                let last = self.last[suffix_gen];
                                let prefix = self.prefix[suffix_gen].expect("Should be present");
                                let first_prefix = self
                                    .get_right_cayley_element(prefix, first)
                                    .expect("Should be present");
                                self.right_cayley_graph[(u, i)] =
                                    self.get_left_cayley_element(first_prefix, last);
                            }
                            None => {
                                self.right_cayley_graph[(u, i)] = Some(first);
                            }
                        }
                    } else {
                        let product = self.elements[u].multiply(&self.generators[i]);
                        match self.element_map.get(&product) {
                            // If we have already seen this element, add a new rule
                            Some(index) => self.right_cayley_graph[(u, *index)] = Some(i),
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
                                self.reduced.add_col();
                                self.reduced[(u, i)] = true;
                                // Update right cayley graph, left cayley graph will be done later
                                self.right_cayley_graph.add_row();
                                self.right_cayley_graph.add_col();
                                self.left_cayley_graph.add_row();
                                self.left_cayley_graph.add_col();
                                // u * a_i = new element
                                self.right_cayley_graph[(u, new_pos)] = Some(i);
                                // TODO remove
                                debug_assert!(self.elements.len() == self.element_map.len());
                                debug_assert!(self.elements.len() == self.first.len());
                                debug_assert!(self.elements.len() == self.last.len());
                                debug_assert!(self.elements.len() == self.prefix.len());
                                debug_assert!(self.elements.len() == self.suffix.len());
                                // Update length, this is simply one more than u
                                self.length.push(self.length[u] + 1);
                                // Update last
                                last = new_pos
                            }
                        }
                    }
                }
                // Go to the next element
                u += 1;
            }
            u = v;
            // Now compute a_i * u to fill in left cayley graph
            while self.length[u] == self.current_word_length {
                let prefix = self.prefix[u].expect("Should be present.");
                let last = self.last[u];
                for i in 0..self.generators.len() {
                    // We work out what a_i * u is from prior information.
                    let res = {
                        let ap = self
                            .get_left_cayley_element(prefix, i)
                            .expect("Should already be computed");
                        self.get_right_cayley_element(ap, last)
                            .expect("Should already be computed.")
                    };
                    // a_i * u = res
                    self.left_cayley_graph[(u, res)] = Some(i);
                }
                // Go onto the next element.
                u += 1;
            }
            v = u;
            self.current_word_length += 1;
            if u == last {
                break;
            }
        }
    }
}

impl<T, U> FroidurePinBuilder<T, U> for FroidurePin<T>
where
    T: SemigroupElement + std::hash::Hash,
    U: Semigroup<T>,
{
    fn new(semigroup: U) {
        FroidurePin::new(semigroup.generators());
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
