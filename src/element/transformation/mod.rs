use std::sync::Arc;

use super::SemigroupElement;

pub mod err;

/// Representation of a transformation on the points 0..n-1
/// This is stored as a vector using the images of each point from 0..n-1
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn from_vec(degree: usize, vals: Vec<usize>) -> Result<Self, err::TransformationError> {
        if vals.len() != degree || !vals.iter().all(|x| *x < degree) {
            Err(err::TransformationError::InvalidImage {
                degree,
                image: vals,
            })
        } else {
            Ok(Transformation::from_vec_unchecked(degree, vals))
        }
    }

    /// Create a Transformation given an image. This does not perform the validation.
    pub(crate) fn from_vec_unchecked(degree: usize, vals: Vec<usize>) -> Self {
        Self {
            degree,
            vals: vals.into(),
        }
    }

    /// Return the identity transformation on degree points
    /// ```
    /// use yatyat::element::transformation::Transformation;
    ///
    /// let id = Transformation::id(5);
    /// assert!(id.is_id())
    /// ```
    pub fn id(degree: usize) -> Self {
        let vals: Vec<_> = if degree > 0 {
            (0..degree - 1).collect()
        } else {
            Vec::with_capacity(0)
        };
        Self {
            degree,
            vals: vals.into(),
        }
    }

    /// Return if a transformation is the identity transformation.
    pub fn is_id(&self) -> bool {
        self.vals.iter().enumerate().all(|(i, x)| i == *x)
    }

    /// Return the degree of the transformation
    /// ```
    /// use yatyat::element::transformation::Transformation;
    ///
    /// let id = Transformation::id(8);
    /// assert_eq!(8_usize, id.degree())
    /// ```
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Apply the transformation to a given point.
    /// If x is less than the degree of the tranformation, then the result of applying the transformation is returned. Otherwise None is returned.
    /// ```
    /// use yatyat::element::transformation::Transformation;
    ///
    /// let f = Transformation::from_vec(3, vec![0, 2, 2]).unwrap();
    /// assert_eq!(2, f.apply(1).unwrap());
    /// assert!(f.apply(5).is_err())
    /// ```
    pub fn apply(&self, x: usize) -> Result<usize, err::TransformationError> {
        if x < self.degree {
            Ok(self.vals[x])
        } else {
            Err(err::TransformationError::InvalidPoint {
                degree: self.degree,
                point: x,
            })
        }
    }

    /// Compose two tranformations.
    /// This is only defined for transformations of the same degree
    /// ```
    /// use yatyat::element::transformation::Transformation;
    ///
    /// let f = Transformation::from_vec(3, vec![0, 2, 2]).unwrap();
    /// let g = Transformation::from_vec(3, vec![2, 1, 0]).unwrap();
    /// let fg = f.multiply(&g).unwrap();
    /// assert_eq!(0, fg.apply(1).unwrap())
    /// ```
    pub fn multiply(&self, other: &Self) -> Result<Self, err::TransformationError> {
        if self.degree == other.degree {
            let vals = (0..self.degree)
                .map(|x| other.apply(self.apply(x).unwrap()).unwrap())
                .collect();
            Ok(Transformation::from_vec_unchecked(self.degree, vals))
        } else {
            Err(err::TransformationError::MismatchingDegree {
                degree1: self.degree,
                degree2: other.degree,
            })
        }
    }
}

impl SemigroupElement for Transformation {
    fn multiply(&self, other: &Self) -> Self {
        // Will panic if degrees do not match
        self.multiply(other).unwrap()
    }
}

impl std::fmt::Display for Transformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        let mut sep = "";
        for (i, x) in self.vals.iter().enumerate() {
            write!(f, "{}{}:{}", sep, i, x)?;
            sep = ", "
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::Transformation;

    #[test]
    fn id() {
        let id0 = Transformation::id(0);
        let id10 = Transformation::id(10);
        let f = Transformation::from_vec(2, vec![1, 1]).unwrap();
        assert!(id0.is_id());
        assert!(id10.is_id());
        assert!(!f.is_id());
    }

    #[test]
    fn invalid_image() {
        // Invalid for value out of range
        let f = Transformation::from_vec(3, vec![0, 0, 4]);
        // Invalid for too many values
        let g = Transformation::from_vec(4, vec![1, 2, 3]);
        assert!(f.is_err());
        assert!(g.is_err());
    }

    #[test]
    fn multiply_inverse() {
        let f = Transformation::from_vec(4, vec![3, 2, 1, 0]).unwrap();
        let f2 = f.multiply(&f).unwrap();
        print!("{:?}", f2);
        assert!(f2.is_id())
    }

    #[test]
    fn multiply() {
        let f = Transformation::from_vec(4, vec![2, 2, 3, 1]).unwrap();
        let g = Transformation::from_vec(4, vec![2, 1, 1, 3]).unwrap();
        let fg = Transformation::from_vec(4, vec![1, 1, 3, 1]).unwrap();
        assert_eq!(fg, f.multiply(&g).unwrap());
    }
}
