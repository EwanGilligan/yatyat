pub mod transformation;

pub trait SemigroupElement: Clone + Eq {
    /// Multipy two elements together, producing a new element
    /// This operation must be associative, but this is not checked.
    fn multiply(&self, other: &Self) -> Self;
}
