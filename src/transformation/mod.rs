use std::sync::Arc;

/// Representation of a transformation on the points 0..n-1
/// This is stored as a vector using the images of each point from 0..n-1
#[derive(Debug, Clone)]
pub struct Transformation {
    degree: usize,
    vals: Arc<[usize]>,
}

impl Transformation {
    pub fn as_vec(&self) -> &[usize] {
        &self.vals[..]
    }

    /// Create transformation from vec of images
    /// This will panic if the points are not defined on the given degree.
    pub fn from_vec(degree: usize, vals: Vec<usize>) -> Self {
        assert!(vals.iter().all(|x| *x < degree));
        Transformation::from_vec_unchecked(degree, vals)
    }

    pub(crate) fn from_vec_unchecked(degree: usize, vals: Vec<usize>) -> Self {
        Self {
            degree,
            vals: vals.into(),
        }
    }

    /// Return the identity transformation on degree points
    pub fn id(degree: usize) -> Self {
        let vals: Vec<_> = (0..degree - 1).collect();
        Self {
            degree,
            vals: vals.into(),
        }
    }

    /// Return the degree of the transformation
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Apply the transformation to a given point.
    /// If x is less than the degree of the tranformation, then the result of applying the transformation is returned. Otherwise None is returned.
    pub fn apply(&self, x: usize) -> Option<usize> {
        if x < self.degree {
            Some(self.vals[x])
        } else {
            None
        }
    }

    /// Compose two tranformations.
    /// This is only defined for transformations of the same degree
    pub fn multiply(&self, other: &Self) -> Option<Self> {
        if self.degree == other.degree {
            let vals = (0..self.degree - 1)
                .map(|x| other.apply(self.apply(x).unwrap()).unwrap())
                .collect();
            Some(Transformation::from_vec_unchecked(self.degree, vals))
        } else {
            None
        }
    }
}
