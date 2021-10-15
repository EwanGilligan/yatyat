use std::fmt::Display;
use std::iter::repeat;
use std::ops::{Index, IndexMut};
use std::vec;

/// Struct to represent a two dimensional array.
/// This is backed by a single vector, which is more efficient than nested vectors.
/// We index by row and then column
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2<T> {
    n_rows: usize,
    n_cols: usize,
    vec: Vec<T>,
}

impl<T> Vec2<T>
where
    T: Default + Clone,
{
    /// Create a new
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        Self {
            n_rows,
            n_cols,
            vec: vec![T::default(); n_rows * n_cols],
        }
    }

    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn n_cols(&self) -> usize {
        self.n_cols
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        let offset = row * self.n_cols;
        &self.vec[(offset..offset + self.n_cols)]
    }

    /// Add a new row to the array, filling with the default value
    pub fn add_row(&mut self) {
        self.n_rows += 1;
        self.vec.extend(repeat(T::default()).take(self.n_cols))
    }

    pub fn add_rows(&mut self, nr: usize) {
        for _ in 0..nr {
            self.add_row();
        }
    }

    /// Add a new column to the array, filling with the default value
    pub fn add_col(&mut self) {
        // Reserve space for one new value in each row
        self.vec.reserve(self.n_rows);
        // Expand each row
        // Iteration performed in reverse to avoid having to update indexing in the loop.
        for row in (0..self.n_rows).rev() {
            // The position at the start of the next row will then be the new end of the previous row.
            self.vec.insert((row + 1) * self.n_cols, T::default())
        }
        self.n_cols += 1;
    }

    pub fn add_cols(&mut self, nr: usize) {
        // Could be made more efficient
        for _ in 0..nr {
            self.add_col();
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.vec[row * self.n_cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.vec[row * self.n_cols + col]
    }
}

impl<T> std::fmt::Display for Vec2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut sep: &str;
        for i in 0..self.n_rows {
            write!(f, "[")?;
            sep = "";
            for j in 0..self.n_cols {
                write!(f, "{}{}", sep, self[(i, j)])?;
                sep = ",";
            }
            write!(f, "]\n")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn initialisation() {
        let vec = Vec2::<usize>::new(5, 6);
        assert!(vec.n_rows() == 5);
        assert!(vec.n_cols() == 6);
    }

    #[test]
    fn setting() {
        let rows = 2;
        let cols = 4;
        let mut vec = Vec2::<usize>::new(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                assert!(vec[(i, j)] == 0);
            }
        }
        for i in 0..rows {
            for j in 0..cols {
                vec[(i, j)] = 1;
                assert!(vec[(i, j)] == 1);
            }
        }
    }

    #[test]
    fn add_row() {
        let mut vec = Vec2::<usize>::new(3, 3);
        assert!(vec.n_rows() == 3);
        assert!(vec.n_cols() == 3);
        vec.add_row();
        assert!(vec.n_rows() == 4);
        assert!(vec.n_cols() == 3);
    }

    #[test]
    fn add_col() {
        let mut vec = Vec2::<usize>::new(3, 3);
        assert!(vec.n_rows() == 3);
        assert!(vec.n_cols() == 3);
        vec[(1, 2)] = 5;
        vec[(0, 1)] = 3;
        vec[(2, 0)] = 1;
        vec.add_col();
        vec[(1, 3)] = 7;
        vec[(2, 3)] = 6;
        assert!(vec.n_rows() == 3);
        assert!(vec.n_cols() == 4);
        assert!(vec.get_row(1) == &[0, 0, 5, 7]);
        assert!(vec.get_row(0) == &[0, 3, 0, 0]);
        assert!(vec.get_row(2) == &[1, 0, 0, 6]);
    }

    #[test]
    fn get_row() {
        let mut vec = Vec2::<usize>::new(3, 3);
        assert!(vec.get_row(2) == &[0, 0, 0]);
        vec[(2, 0)] = 5;
        vec[(2, 1)] = 4;
        vec[(2, 2)] = 6;
        assert!(vec.get_row(2) == &[5, 4, 6]);
    }
}
