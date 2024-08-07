//! # Rustrix
//! 
//! Supports macro and basic operations for matrix.\
//! Please note that safety for overflow or other edge cases is not tested.
//! 
//! - Matrices can now contain generic type values.
//!   (i32 and f64 are tested.)
//! 
//! ## Initialization
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let mx = mx![
//!     1, 2, 3;
//!     4, 5, 6;
//! ];
//! ```
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let (rows, cols, initial_value) = (2, 3, 1);
//! let mx = mx!(rows, cols; initial_value);
//! 
//! // 1 1 1
//! // 1 1 1
//! ```
//! 
//! ## Add
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m1 = mx!(3, 3; 2);
//! let m2 = mx!(3, 3; 3);
//! let mx = m1 + m2;
//! 
//! // 5 5 5
//! // 5 5 5
//! // 5 5 5
//! ```
//! 
//! ## Subtract
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m1 = mx!(3, 3; 2);
//! let m2 = mx!(3, 3; 3);
//! let mx = m1 - m2;
//! 
//! // -1 -1 -1
//! // -1 -1 -1
//! // -1 -1 -1
//! ```
//! 
//! ## Dot product
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m1 = mx![
//!     1, 1, 1;
//!     2, 2, 2;
//! ];
//! 
//! let m2 = mx![
//!     1, 1, 1, 1;
//!     2, 2, 2, 2;
//!     3, 3, 3, 3;
//! ];
//! 
//! let mx = m1 * m2;
//! 
//! //  6  6  6  6
//! // 12 12 12 12
//! ```
//! 
//! ## Transpose
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let mx = mx![
//!     1, 2;
//!     3, 4;
//!     5, 6;
//! ];
//! 
//! let tp = mx.tp();
//! 
//! // 1 3 5
//! // 2 4 6
//! ```

use std::{
    iter::Zip,
    ops::{self, Add, AddAssign, Mul, Sub, SubAssign},
    slice::Iter,
};

type Vec2D<T> = Vec<Vec<T>>;

// TODO: How to ensure the vector is not empty?
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T>(pub Vec2D<T>)
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>;

/// ```
/// use rustrix::*;
/// 
/// // Both macros build the same results.
/// let (rows, cols, initial_value) = (2, 3, 1);
/// let m1 = mx!(rows, cols; initial_value);
/// let m2 = mx![
///     1, 1, 1;
///     1, 1, 1;
/// ];
/// ```
#[macro_export]
macro_rules! mx {
    ($r: expr, $c: expr$(; $v: expr)?) => {
        Matrix::from(vec![vec![0$(+$v)?; $c]; $r])
    };
    [$($($v: expr),+);+$(;)?] => {
        Matrix::from(vec![$(vec![$($v,)+]),+])
    };
}


impl<T> From<Vec2D<T>> for Matrix<T>
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>,
{
    fn from(v: Vec2D<T>) -> Self {
        Matrix(v)
    }
}

impl<T> Matrix<T>
where
    T: Clone + Copy
        + Add<Output = T> + Sub<Output = T> + Mul<Output = T>
        + AddAssign + SubAssign
        + From<i32>,
{
    /// Returns the number of rows in the matrix.
    pub fn rows(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of columns in the matrix.
    pub fn cols(&self) -> usize {
        self.0[0].len()
    }

    /// Returns the value at given row, column.
    pub fn get(&self, row: usize, col: usize) -> T {
        self.0[row][col]
    }

    /// Sets the value at given row, column.
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.0[row][col] = value;
    }

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

fn zip<'a, T>(v1: &'a Vec<T>, v2: &'a Vec<T>) -> Zip<Iter<'a, T>, Iter<'a, T>> {
    Iterator::zip(v1.iter(), v2.iter())
}

impl<T> ops::Add for Matrix<T>
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


#[cfg(test)]
mod test_matrix {
    use super::*;

    #[test]
    fn test_macro_1() {
        let mx = mx!(2, 3; 1);

        assert_eq!(mx, Matrix(vec![vec![1, 1, 1], vec![1, 1, 1]]));
    }

    #[test]
    fn test_macro_1_2() {
        let mut mx = mx!(2, 3; 0);
        mx.0[0][0] = 1;
        assert_ne!(mx.0[0][0], mx.0[0][1]);
    }

    #[test]
    fn test_macro_2() {
        let mx = mx![
            0, 0, 0;
            0, 0, 0;
        ];

        assert_eq!(mx, Matrix(vec![vec![0, 0, 0], vec![0, 0, 0]]));
    }

    #[test]
    fn test_transpose() {
        let mx = mx![
            1, 2, 3;
            4, 5, 6;
        ];

        let tp = mx![
            1, 4;
            2, 5;
            3, 6;
        ];

        assert_eq!(mx.transpose(), tp);
    }

    #[test]
    fn test_add_i32() {
        let m1 = mx![
            1, 1;
            1, 1;
        ];

        let m2 = mx![
            1, 2;
            3, 4;
        ];

        let m3 = mx![
            2, 3;
            4, 5;
        ];

        assert_eq!(m1 + m2, m3);
    }

    #[test]
    fn test_add_f64() {
        let m1 = mx![
            1.0, 1.0;
            1.0, 1.0;
        ];

        let m2 = mx![
            1.0, 2.0;
            3.0, 4.0;
        ];

        let m3 = mx![
            2.0, 3.0;
            4.0, 5.0;
        ];

        assert_eq!(m1 + m2, m3);
    }

    #[test]
    fn test_sub()  {
        let m1 = mx![
            3, 2, 1;
        ];

        let m2 = mx![
            1, 1, 1;
        ];

        let m3 = mx![
            2, 1, 0;
        ];

        assert_eq!(m1 - m2, m3);
    }

    #[test]
    fn test_dot_prod_i32() {
        let m1 = mx![
            1, 1, 1;
            1, 1, 1;
        ];

        let m2 = mx![
            2, 2, 2;
            2, 2, 2;
            2, 2, 2;
        ];

        let mx = mx![
            6, 6, 6;
            6, 6, 6;
        ];

        assert_eq!(m1 * m2, mx);
    }

    #[test]
    fn test_dot_prod_f64() {
        let m1 = mx![
            1.0, 1.0, 1.0;
            1.0, 1.0, 1.0;
        ];

        let m2 = mx![
            2.0, 2.0, 2.0;
            2.0, 2.0, 2.0;
            2.0, 2.0, 2.0;
        ];

        let mx = mx![
            6.0, 6.0, 6.0;
            6.0, 6.0, 6.0;
        ];

        assert_eq!(m1 * m2, mx);
    }
}