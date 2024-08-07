use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

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
/// // Both macro invocations build the same results.
/// let (rows, cols, init) = (2, 3, 1);
/// let m1 = mx!(rows, cols; init);
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
}

mod ops;

#[cfg(test)]
mod tests {
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