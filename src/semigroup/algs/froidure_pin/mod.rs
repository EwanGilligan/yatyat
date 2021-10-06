use crate::{
    element::{self, SemigroupElement},
    semigroup::{word::Word, Semigroup},
};

mod simple;

struct FroidurePinResult<U>
where
    U: SemigroupElement,
{
    // Elements sorted in military order
    elements: Vec<U>,
    // TODO add left and right Cayley graphs
}
