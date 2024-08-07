use std::{iter::Zip, slice::Iter};
use super::*;

fn zip<'a, T>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> Zip<Iter<'a, T>, Iter<'a, T>> {
    Iterator::zip(v1.iter(), v2.iter())
}

impl<T> Add for Matrix<T>
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Self) -> Self {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Cannot add matrices with different sizes.");
        }

        zip(&self.0, &rhs.0).map(|(r1, r2)| {
            zip(r1, r2).map(|(&v1, &v2)| {
                v1 + v2
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>().into()

    }
}

impl<T> ops::Sub for Matrix<T>
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self {
        if self.rows() != rhs.rows() || self.cols() != rhs.cols() {
            panic!("Cannot subtract matrices with different sizes.");
        }

        zip(&self.0, &rhs.0).map(|(r1, r2)| {
            zip(r1, r2).map(|(&v1, &v2)| {
                v1 - v2
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>().into()
    }
}

impl<T> ops::Mul for Matrix<T>
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>,
{
    type Output = Matrix<T>;

    /// Performs the matrix dot product operation.
    fn mul(self, rhs: Self) -> Self {
        if self.cols() != rhs.rows() {
            panic!("Number of columns in lhs and number of rows in rhs differs.");
        }

        let rows = self.rows();
        let cols = rhs.cols();
        let terms = self.cols();

        (0..rows).map(|r| {
            (0..cols).map(|c| {
                (0..terms).fold(0.into(), |acc, t| {
                    acc + self.0[r][t] * rhs.0[t][c]
                })
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>().into()
    }
}

impl<T> Matrix<T>
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>,
{
    /// Returns a transposed matrix of the original matrix.
    pub fn transpose(&self) -> Self {
        (0..self.cols()).map(|j| {
            (0..self.rows()).map(|i| {
                self.0[i][j]
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>().into()
    }

    /// Alias for Matrix::transpose.
    pub fn tp(&self) -> Self {
        self.transpose()
    }

    
    #[deprecated(since="0.2.0", note="Please use '*' operator instead.")]
    /// Performs the matrix dot product operation.
    pub fn dot_prod(m1: Self, m2: Self) -> Self {
        m1 * m2
    }
}