use crate::{
    element::SemigroupElement,
    semigroup::{word::Word, Semigroup},
    utils::vec2::Vec2,
    DetHashMap,
};

mod froidure_pin_impl;
mod simple;

type CayleyGraphType = Vec2<Option<usize>>;

#[derive(Debug)]
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

trait FroidurePinBuilder<T>
where
    T: SemigroupElement,
{
    fn new<U>(semigroup: &U) -> Self
    where
        U: Semigroup<T>;
    fn build(self) -> FroidurePinResult<T>;
}

/// Macro for testing multiple implementations.

macro_rules! froidure_pin_test {
    ($test_impl:ty, $name:ident) => {
        #[cfg(test)]
        mod $name {
            use super::*;

            use crate::element::transformation::Transformation;
            use crate::semigroup::impls::transformation::TransformationSemigroup;

            #[test]
            fn trivial_monoid() {
                // Trivial element for transformations of degree 3
                let s = TransformationSemigroup::new(&[
                    Transformation::from_vec(3, vec![0, 1, 2]).unwrap()
                ])
                .unwrap();
                let fp = <$test_impl>::new(&s);
                let res = fp.build();
                dbg!(&res);
                assert!(res.elements.len() == 1);
            }

            #[test]
            fn symmetric_group_5() {
                let s = TransformationSemigroup::new(&[
                    Transformation::from_vec(5, vec![1, 0, 2, 3, 4]).unwrap(),
                    Transformation::from_vec(5, vec![0, 2, 3, 4, 1]).unwrap(),
                ])
                .unwrap();
                let fp = <$test_impl>::new(&s);
                let res = fp.build();
                assert!(res.elements.len() == 120);
            }

            #[test]
            fn paper_example() {
                let s = TransformationSemigroup::new(&[
                    Transformation::from_vec(6, vec![1, 1, 3, 3, 4, 5]).unwrap(),
                    Transformation::from_vec(6, vec![4, 2, 3, 3, 5, 5]).unwrap(),
                ])
                .unwrap();
                let fp = <$test_impl>::new(&s);
                let res = fp.build();
                assert!(res.elements.len() == 7);
            }
        }

        //let s = TransformationSemigroup::new(&[
        //         Transformation::from_vec(8, vec![1, 7, 2, 6, 0, 4, 1, 5]).unwrap(),
        //         Transformation::from_vec(8, vec![2, 4, 6, 1, 4, 5, 2, 7]).unwrap(),
        //         Transformation::from_vec(8, vec![3, 0, 7, 2, 4, 6, 2, 4]).unwrap(),
        //         Transformation::from_vec(8, vec![3, 2, 3, 4, 5, 3, 0, 1]).unwrap(),
        //         Transformation::from_vec(8, vec![4, 3, 7, 7, 4, 5, 0, 4]).unwrap(),
        //         Transformation::from_vec(8, vec![5, 6, 3, 0, 3, 0, 5, 1]).unwrap(),
        //         Transformation::from_vec(8, vec![6, 0, 1, 1, 1, 6, 3, 4]).unwrap(),
        //         Transformation::from_vec(8, vec![7, 7, 4, 0, 6, 4, 1, 7]).unwrap(),
        //     ])
    };
}

froidure_pin_test!(
    froidure_pin_impl::FroidurePin<Transformation>,
    froidure_pin_test
);
froidure_pin_test!(simple::FroidurePinSimple<Transformation>, simple_test);
