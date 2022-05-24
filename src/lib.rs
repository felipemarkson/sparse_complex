/*!
An simple solver for sparse complex linear systems based on [Eigen::SparseLU](https://eigen.tuxfamily.org/dox/classEigen_1_1SparseLU.html).

## Complex Number representation

We use [num::Complex<T>](https://docs.rs/num/latest/num/struct.Complex.html) to represent complex numbers. See [num](https://docs.rs/num/latest/num/) crate for more information.

## Example

Lets consider the complex linear system bellow:
```math
\begin{bmatrix}
1 - j1 & 0\\
0 & -1 + j1
\end{bmatrix}
\begin{bmatrix}
x_1 \\
x_2
\end{bmatrix}=
\begin{bmatrix}
1 \\
j1
\end{bmatrix}
```

We can solve this system as follows:

```rust
use num::Complex;
use sparse_complex::ComplexMatrix;

let mut m = ComplexMatrix::<f64>::new();
m.add_element(0, 0, Complex { re: 1., im: -1. });
m.add_element(1, 1, Complex { re: -1., im: 1. });
let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
m.solve(&mut b).unwrap();

let expected = vec![Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)];
assert_eq!(b, expected);
```

The solution of this system is:
```math
\frac{1}{2}
\begin{bmatrix}
1 + j1 \\
1 - j1
\end{bmatrix}
```

## Version Compatible
The ```sparse_complex``` crate is tested for rustc 1.61 and greater.

*/
use num::complex::Complex;
use num_traits::float::Float;
use std::fmt;
mod solver;

/// The complex matrix struct
#[derive(Clone, PartialEq)]
pub struct ComplexMatrix<T: Float> {
    entries: Vec<Complex<T>>,
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl<T: Float> ComplexMatrix<T> {
    /// Create a new, initially empty ```ComplexMatrix```
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let mut m = ComplexMatrix::<f64>::new();
    ///```
    pub fn new() -> Self {
        ComplexMatrix {
            entries: vec![],
            rows: vec![],
            cols: vec![],
        }
    }

    /// Create a new, initially empty ```ComplexMatrix``` with a given capacity
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let mut m = ComplexMatrix::<f64>::with_capacity(5);
    ///```
    pub fn with_capacity(capacity: usize) -> Self {
        ComplexMatrix {
            entries: Vec::with_capacity(capacity),
            rows: Vec::with_capacity(capacity),
            cols: Vec::with_capacity(capacity),
        }
    }

    /// Create a new ```ComplexMatrix``` from a vector of ```(row, col, Complex<T>)``` entries.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// use num::Complex;
    /// let entries = vec![(0, 0, Complex::new(1., 1.)), (1, 1, Complex::new(1., 1.))];
    /// let mut m = ComplexMatrix::<f64>::from_entries(entries);
    ///```
    pub fn from_entries(entries: Vec<(usize, usize, Complex<T>)>) -> Self {
        let mut m = ComplexMatrix::with_capacity(entries.len());

        for (row, col, value) in entries {
            m.add_element(row, col, value);
        }
        m
    }

    /// Add or set an element at location ```(row, col)``` with value.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// use num::Complex;
    /// 
    /// let Z1: Complex<f64> = Complex { re: 1., im: -1. };
    /// let Z2: Complex<f64> = Complex { re: -1., im: 1. };
    /// 
    /// let mut m = ComplexMatrix::new();
    /// m.add_element(0, 0, Z1);
    /// m.add_element(1, 1, Z2);
    /// 
    /// assert_eq!(m.get(0, 0), Some(&Z1));
    /// assert_eq!(m.get(1, 1), Some(&Z2));
    ///```
    pub fn add_element(&mut self, row: usize, col: usize, value: Complex<T>) {
        self.entries.push(value);
        self.rows.push(row);
        self.cols.push(col);
    }

    ///  Returns the Element-value at ```(row, col)``` if present, or None if not.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// use num::Complex;
    /// 
    /// let Z1: Complex<f64> = Complex { re: 1., im: -1. };
    /// let Z2: Complex<f64> = Complex { re: -1., im: 1. };
    /// 
    /// let mut m = ComplexMatrix::new();
    /// m.add_element(0, 0, Z1);
    /// m.add_element(1, 1, Z2);
    /// 
    /// assert_eq!(m.get(0, 0), Some(&Z1));
    /// assert_eq!(m.get(1, 1), Some(&Z2));
    ///```
    pub fn get(&self, row: usize, col: usize) -> Option<&Complex<T>> {
        self.rows
            .iter()
            .zip(self.cols.iter())
            .zip(self.entries.iter())
            .find(|&((r, c), _)| *r == row && *c == col)
            .map(|(_, v)| v)
    }
}

impl ComplexMatrix<f64> {
    /// Solve the system `Ax=b`, where:
    /// * `A` is a complex matrix
    /// * `b` is a complex vector
    ///
    /// Returns a `Result`. `Ok(())` if the system was solved successfully, `Err(String)` if not. 
    /// The result is stored in `b`.
    ///
    /// The solution use the [Eigen::SparseLU](https://eigen.tuxfamily.org/dox/classEigen_1_1SparseLU.html).
    ///
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// use num::Complex;
    /// 
    /// let Z1: Complex<f64> = Complex { re: 1., im: -1. };
    /// let Z2: Complex<f64> = Complex { re: -1., im: 1. };
    /// 
    /// let mut m = ComplexMatrix::new();
    /// m.add_element(0, 0, Z1);
    /// m.add_element(1, 1, Z2);
    /// 
    /// let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
    /// m.solve(&mut b).unwrap();
    /// 
    /// let expected = vec![Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)];
    /// assert_eq!(b, expected);
    ///```
    pub fn solve(&self, b: &mut [Complex<f64>]) -> Result<(), &'static str> {
        unsafe {
            solver::solve_cpp(
                self.entries.as_ptr(),
                self.rows.as_ptr(),
                self.cols.as_ptr(),
                self.entries.len(),
                b.as_mut_ptr(),
                b.len(),
            )
        }

        Ok(())
    }
}

impl ComplexMatrix<f32> {
    /// Solve the system `Ax=b`, where:
    /// * `A` is a complex matrix
    /// * `b` is a complex vector
    ///
    /// Returns a `Result`. `Ok(())` if the system was solved successfully, `Err(String)` if not. 
    /// The result is stored in `b`.
    ///
    /// This solution use the [Eigen::SparseLU](https://eigen.tuxfamily.org/dox/classEigen_1_1SparseLU.html).
    ///
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// use num::Complex;
    /// 
    /// let Z1: Complex<f32> = Complex { re: 1., im: -1. };
    /// let Z2: Complex<f32> = Complex { re: -1., im: 1. };
    /// 
    /// let mut m = ComplexMatrix::new();
    /// m.add_element(0, 0, Z1);
    /// m.add_element(1, 1, Z2);
    /// 
    /// let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
    /// m.solve(&mut b).unwrap();
    /// 
    /// let expected = vec![Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)];
    /// assert_eq!(b, expected);
    ///```
    pub fn solve(&self, b: &mut [Complex<f32>]) -> Result<(), &'static str> {
        unsafe {
            solver::solve_cpp32(
                self.entries.as_ptr(),
                self.rows.as_ptr(),
                self.cols.as_ptr(),
                self.entries.len(),
                b.as_mut_ptr(),
                b.len(),
            )
        }

        Ok(())
    }
}

impl<T: Float + std::fmt::Display> fmt::Debug for ComplexMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::from("ComplexMatrix { \n");
        let elements = self
            .rows
            .iter()
            .zip(self.cols.iter())
            .zip(self.entries.iter());
        for ((row, col), value) in elements {
            msg = format!("{}  ({},{}) -> {}\n", msg, row, col, value)
        }
        write!(f, "{}}}", msg)
    }
}

// #[cfg(test)]
// mod tests_simple_matrix {
//     use super::*;
//     const Z1: Complex<f64> = Complex { re: 1., im: -1. };
//     const Z2: Complex<f64> = Complex { re: -1., im: 1. };

//     const Z1_32: Complex<f32> = Complex { re: 1., im: -1. };
//     const Z2_32: Complex<f32> = Complex { re: -1., im: 1. };

//     #[test]
//     fn test_add_element() {
//         let mut m = ComplexMatrix::new();
//         m.add_element(0, 0, Z1);
//         m.add_element(1, 1, Z2);

//         assert_eq!(*m.get(0, 0).unwrap(), Z1);
//         assert_eq!(*m.get(1, 1).unwrap(), Z2);
//     }

//     #[test]
//     fn test_from_elements() {
//         let entries = vec![(0, 0, Z1), (1, 1, Z2)];
//         let m = ComplexMatrix::from_entries(entries);
//         assert_eq!(*m.get(0, 0).unwrap(), Z1);
//         assert_eq!(*m.get(1, 1).unwrap(), Z2);
//     }

//     #[test]
//     fn test_solve() {
//         let mut m = ComplexMatrix::new();
//         m.add_element(0, 0, Z1);
//         m.add_element(1, 1, Z2);
//         let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
//         m.solve(&mut b).unwrap();

//         let expected = vec![Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)];
//         assert_eq!(b, expected);
//     }

//     #[test]
//     fn test_solve32() {
//         let mut m = ComplexMatrix::new();
//         m.add_element(0, 0, Z1_32);
//         m.add_element(1, 1, Z2_32);
//         let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
//         m.solve(&mut b).unwrap();

//         let expected = vec![Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)];
//         assert_eq!(b, expected);
//     }
// }
