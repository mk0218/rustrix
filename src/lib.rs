//! Supportsc macro and basic operations for matrix.
//! Only integer items are available currently.

//! ## Initialization
//! ```
//! use rustrix::*;
//! 
//! let mx = mx![
//!     1, 2, 3;
//!     4, 5, 6;
//! ];
//! ```
//! ```
//! use rustrix::*;
//! 
//! let (rows, cols, initial_value) = (2, 3, 1);
//! let mx = mx!(rows, cols; initial_value);
//! // 1 1 1
//! // 1 1 1
//! ```

//! ## Add
//! ```
//! use rustrix::*;
//! 
//! let m1 = mx!(3, 3; 2);
//! let m2 = mx!(3, 3; 3);
//! let mx = m1 + m2;
//! // 5 5 5
//! // 5 5 5
//! // 5 5 5
//! ```

//! ## Subtract
//! ```
//! use rustrix::*;
//! 
//! let m1 = mx!(3, 3; 2);
//! let m2 = mx!(3, 3; 3);
//! let mx = m1 - m2;
//! // -1 -1 -1
//! // -1 -1 -1
//! // -1 -1 -1
//! ```

//! ## Dot product
//! ```
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

//! ## Transpose
//! ```
//! use rustrix::*;
//! 
//! let mx = mx![
//!     1, 2;
//!     3, 4;
//!     5, 6;
//! ];
//! 
//! let tp = mx.tp();
//! // 1 3 5
//! // 2 4 6
//! ```

use std::ops;

// TODO: How to ensure the vector is not empty?
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

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
    ($r: expr, $c: expr $(; $v: expr)?) => {
        {
            let mut v = 0;

            $( v = $v; )?
            
            let mut mx: Vec<Vec<i32>> = vec![];
            for r in 0..$r {
                mx.push(vec![]);
                
                for _ in 0..$c {
                    mx[r].push(v);
                }
            }

            Matrix(mx)
        }
    };
    [$($($v: expr),+);+$(;)?] => {
        {
            let mut mx: Vec<Vec<i32>> = vec![];
            $(
                let mut row: Vec<i32> = vec![];
                $(
                    row.push($v);
                )+
                mx.push(row);
            )+

            Matrix(mx)
        }
    };
}

impl Matrix {
    /// Returns the number of rows in the matrix.
    pub fn rows(&self) -> usize {
        self.0.len()
    }

    /// Returns the number of columns in the matrix.
    pub fn cols(&self) -> usize {
        self.0[0].len()
    }

    /// Returns a transposed matrix of the original matrix.
    pub fn transpose(&self) -> Self {
        let mut mx: Vec<Vec<i32>> = vec![];

        for c in 0..self.cols() {
            mx.push(vec![]);
            for r in 0..self.rows() {
                mx[c].push(self.0[r][c]);
            }
        }

        Matrix(mx)
    }

    /// Alias for Matrix::transpose.
    pub fn tp(&self) -> Self {
        self.transpose()
    }

    /// Performs the matrix dot product operation.
    /// Same as the * operator.
    pub fn dot_prod(m1: Self, m2: Self) -> Self {
        if m1.cols() != m2.rows() {
            panic!("Number of columns in m1 and number of rows in m2 differs.");
        }

        let rows = m1.rows();
        let cols = m2.cols();
        let terms = m1.cols();

        let mut mx = mx!(rows, cols; 0);

        for r in 0..rows {
            for c in 0..cols {
                for t in 0..terms {
                    mx.0[r][c] += m1.0[r][t] * m2.0[t][c];
                }
            }
        }

        mx
    }
}

impl ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Self) -> Self {
        if self.rows() != _rhs.rows() || self.cols() != _rhs.cols() {
            panic!("Cannot add matrices with different sizes.");
        }

        let mut output = self.clone();

        for r in 0..output.rows() {
            for c in 0..output.cols() {
                output.0[r][c] += _rhs.0[r][c];
            }
        }
        
        output
    }
}

impl ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, _rhs: Self) -> Self {
        if self.rows() != _rhs.rows() || self.cols() != _rhs.cols() {
            panic!("Cannot subtract matrices with different sizes.");
        }

        let mut output = self.clone();

        for r in 0..output.rows() {
            for c in 0..output.cols() {
                output.0[r][c] -= _rhs.0[r][c];
            }
        }
        
        output
    }
}

impl ops::Mul for Matrix {
    type Output = Matrix;

    /// Performs the matrix dot product operation.
    /// Same as `dot_prod()`.
    fn mul(self, _rhs: Self) -> Self {
        Matrix::dot_prod(self,  _rhs)
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
    fn test_add() {
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
    fn test_dot_prod() {
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
}