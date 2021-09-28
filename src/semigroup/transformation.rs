use std::fmt::Display;

use crate::element::transformation::{err::TransformationError, Transformation};

use super::Semigroup;

/// Struct that represents a transformation semigroup
pub struct TransformationSemigroup {
    degree: usize,
    generators: Vec<Transformation>,
}

impl TransformationSemigroup {
    /// Create a new TransformationSemigroup from a list of generators.
    /// The new generators must have the same degree, otherwise an error is returned.
    pub fn new(gens: &[Transformation]) -> Result<Self, TransformationError> {
        // Take degree of first element as degree of Transformation Semigroup. Need to handle trivial case.
        let degree = gens.get(0).map(|f| f.degree()).unwrap_or(0);
        // Must have same degree for all values
        if let Some(f) = gens.iter().skip(1).find(|f| f.degree() != degree) {
            Err(TransformationError::MismatchingDegree {
                degree1: degree,
                degree2: f.degree(),
            })
        } else {
            Ok(TransformationSemigroup {
                degree,
                generators: gens.to_vec(),
            })
        }
    }

    /// Return the degree of the transformations in this Semigroup
    pub fn degree(&self) -> usize {
        self.degree
    }
}

impl Display for TransformationSemigroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<")?;
        let mut sep = "";
        for gen in self.generators() {
            write!(f, "{}{}", sep, gen)?;
            sep = ", "
        }
        write!(f, ">")
    }
}

impl Semigroup<Transformation> for TransformationSemigroup {
    fn generators(&self) -> &[Transformation] {
        &self.generators[..]
    }
}

#[cfg(test)]
mod test {
    use crate::element::transformation::Transformation;

    use super::TransformationSemigroup;

    #[test]
    fn trivial_semigroup() {
        assert!(TransformationSemigroup::new(&[]).is_ok());
    }

    #[test]
    fn valid_gens() {
        let f = Transformation::from_vec(5, vec![2, 2, 3, 1, 4]).unwrap();
        let g = Transformation::from_vec(5, vec![2, 1, 1, 3, 2]).unwrap();
        let s = TransformationSemigroup::new(&[f, g]);
        assert!(s.is_ok());
        println!("{}", s.unwrap());
    }

    #[test]
    fn invalid_gens() {
        let f = Transformation::from_vec(4, vec![2, 2, 3, 1]).unwrap();
        let g = Transformation::from_vec(5, vec![2, 1, 1, 3, 2]).unwrap();
        let s = TransformationSemigroup::new(&[f, g]);
        assert!(s.is_err());
    }
}
