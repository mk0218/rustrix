//! Supports macro and basic operations for matrix.
//! 
//! ## Initialization
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m = mx![
//!     1, 2, 3;
//!     4, 5, 6;
//! ];
//! 
//! let (rows, cols, init) = (2, 3, 1);
//! let m1 = mx!(rows, cols; init);
//! let m2 = mx![
//!     1, 1, 1;
//!     1, 1, 1;
//! ];
//! 
//! assert_eq!(m1, m2)
//! ```
//! 
//! ## Add
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m = mx!(2, 3; 2) + mx!(2, 3; 3);
//! ```
//! 
//! ## Subtract
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m = mx!(2, 3; 4) - mx!(2, 3; 1);
//! ```
//! 
//! ## Dot product
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m1 = mx![
//!     1, 1;
//!     2, 2;
//! ];
//! 
//! let m2 = mx![
//!     1, 1, 1, 1;
//!     2, 2, 2, 2;
//! ];
//! 
//! let m = m1 * m2;
//! ```
//! 
//! ## Transpose
//! 
//! ```rust
//! use rustrix::*;
//! 
//! let m1 = mx![
//!     1, 2;
//!     3, 4;
//!     5, 6;
//! ];
//! 
//! let m2 = m1.tp();
//! let m3 = m1.transpose();
//! ```

mod matrix;
pub use matrix::Matrix;
