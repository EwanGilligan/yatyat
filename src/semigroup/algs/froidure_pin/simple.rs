use super::{CayleyGraphType, FroidurePinBuilder, FroidurePinResult};
use itertools::Itertools;
use std::hash::Hash;

use crate::{
    element::SemigroupElement,
    semigroup::{
        word::{Alphabet, Word},
        Semigroup,
    },
    utils::vec2::Vec2,
    DetHashMap,
};

pub struct FroidurePinSimple<T>
where
    T: SemigroupElement + Hash,
{
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
}

impl<T> FroidurePinSimple<T>
where
    T: SemigroupElement + Hash,
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
        // Add to element map
        for (idx, elem) in elements.iter().enumerate() {
            element_map.insert(elem.clone(), idx);
        }
        let rewrite_rules = Vec::new();
        // 2d arrays for the Cayley graphs
        let mut left_cayley_graph = Vec2::new(elements.len(), elements.len());
        let mut right_cayley_graph = Vec2::new(elements.len(), elements.len());
        // Initialise identity for the graphs
        for i in 1..=generators.len() {
            left_cayley_graph[(i, 0)] = Some(i);
            left_cayley_graph[(0, i)] = Some(i);
            right_cayley_graph[(i, 0)] = Some(i);
            right_cayley_graph[(0, i)] = Some(i);
        }
        FroidurePinSimple {
            generators,
            elements,
            element_map,
            rewrite_rules,
            left_cayley_graph,
            right_cayley_graph,
        }
    }

    fn run(&mut self) {
        // If we only have the identity then we have no work to do.
        if self.elements.len() == 1 {
            return;
        }
        let mut u = 1;
        loop {
            for gen in 1..=self.generators.len() {
                let product = self.elements[u].multiply(&self.elements[gen]);
                // If we find a new element
                match self.element_map.get(&product) {
                    // Element has already been found.
                    Some(&idx) => {
                        //TODO add new rule
                        self.right_cayley_graph[(u, gen)] = Some(idx);
                    }
                    // We've found a new element
                    None => {
                        let new_pos = self.elements.len();
                        self.element_map.insert(product.clone(), new_pos);
                        self.elements.push(product);
                        // Need a new row in the cayley graphs
                        self.right_cayley_graph.add_row();
                        self.left_cayley_graph.add_row();
                        self.right_cayley_graph[(u, gen)] = Some(new_pos);
                    }
                }
            }
            u += 1;
            // End if u has no successors
            if u == self.elements.len() {
                break;
            }
        }
    }
}

impl<T> FroidurePinBuilder<T> for FroidurePinSimple<T>
where
    T: SemigroupElement + std::hash::Hash + std::fmt::Debug,
{
    fn new<U>(semigroup: &U) -> Self
    where
        U: Semigroup<T>,
    {
        FroidurePinSimple::new(semigroup)
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
