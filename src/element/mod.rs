pub mod transformation;

pub trait SemigroupElement: Clone + Eq {
    /// Multipy two elements together, producing a new element
    /// This operation must be associative, but this is not checked.
    fn multiply(&self, other: &Self) -> Self;
    /// Check if this element is the identity for the monoid of this element.
    /// False if no identity exists.
    fn is_id(&self) -> bool {
        false
    }
}
