use crate::{
    element::SemigroupElement,
    semigroup::{word::Word, Semigroup},
    utils::vec2::Vec2,
    DetHashMap,
};

mod froidure_pin_impl;
mod simple;

type CayleyGraphType = Vec2<Option<usize>>;

struct FroidurePinResult<U>
where
    U: SemigroupElement,
{
    generators: Vec<U>,
    // Elements sorted in military order
    elements: Vec<U>,
    // Map of elements to position in gens
    element_map: DetHashMap<U, usize>,
    // Rewrite rules that index into the elements
    rewrite_rules: Vec<(Word<usize>, Word<usize>)>,
    // The left and right Cayley graphs, which index into the elements.
    left_cayley_graph: CayleyGraphType,
    right_cayley_graph: CayleyGraphType,
}

trait FroidurePinBuilder<T, U>
where
    U: Semigroup<T>,
    T: SemigroupElement,
{
    fn new(semigroup: &U) -> Self;
    fn build(self) -> FroidurePinResult<T>;
}
