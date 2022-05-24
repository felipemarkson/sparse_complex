/*!
An abstraction layer for [sparse21](https://crates.io/crates/sparse21/) that adds support for complex sparse matrices.

## Complex Number representation

In this implementation, a complex number is represented as a tuple of ```f64```.
Where the first element is the real part and the second is the imaginary part, as shown bellow:

```rust
let complex_number: (f64, f64) = (1.0 , 1.0); // 1 + j1
```
The use of ```f64``` is a limitation of [sparse21](https://crates.io/crates/sparse21/).


## Example

Lets consider the complex linear system bellow:
<!-- $$
\begin{bmatrix}
1 + j1 & 0\\
0 & 1+ j1
\end{bmatrix}
\begin{bmatrix}
x_1 \\
x_2
\end{bmatrix}=
\begin{bmatrix}
1 \\
j1
\end{bmatrix}
$$ -->

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=%5Cbegin%7Bbmatrix%7D%0A1%20%2B%20j1%20%26%200%5C%5C%0A0%20%26%201%2B%20j1%0A%5Cend%7Bbmatrix%7D%0A%5Cbegin%7Bbmatrix%7D%0Ax_1%20%5C%5C%0Ax_2%0A%5Cend%7Bbmatrix%7D%3D%0A%5Cbegin%7Bbmatrix%7D%0A1%20%5C%5C%0Aj1%0A%5Cend%7Bbmatrix%7D"></div>

We can solve this system as follows:

```rust
use sparse_complex::ComplexMatrix;

let mut a = ComplexMatrix::new();
a.add_element(0, 0, (1., 1.));
a.add_element(1, 1, (1., 1.));

let b = [(1., 0.), (0., 1.)];

let solution = a.solve(&b);
assert_eq!(solution.unwrap(), vec![(0.5, -0.5), (0.5, 0.5)]);
```

The solution of this system is:
<!-- $$
\frac{1}{2}
\begin{bmatrix}
1 -j1 \\
1 + j1
\end{bmatrix}
$$ -->
<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=%5Cfrac%7B1%7D%7B2%7D%0A%5Cbegin%7Bbmatrix%7D%0A1%20-j1%20%5C%5C%0A1%20%2B%20j1%0A%5Cend%7Bbmatrix%7D"></div>

## Vesion Compatible
The ```sparse_complex``` crate is tested for rustc 1.50 and greater.

*/
use num::{complex::Complex, Num, Zero};
use num_traits::float::Float;
use std::fmt;
mod solver;
mod tools;

#[derive(Clone, Copy, PartialEq)]
pub struct Entry<T: Float> {
    pub row: usize,
    pub col: usize,
    pub value: Complex<T>,
}
impl<T: Float> Entry<T> {
    pub fn new(row: usize, col: usize, value: Complex<T>) -> Entry<T> {
        Entry { row, col, value }
    }
}

impl fmt::Debug for Entry<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {:?})", self.row, self.col, self.value)
    }
}

/// The complex matrix struct
#[derive(Clone, PartialEq)]
pub struct ComplexMatrix<T: Float> {
    entries: Vec<Entry<T>>,
    order: usize,
}

impl<T: Float> ComplexMatrix<T> {
    /// Create a new, initially empty ```ComplexMatrix```
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let mut m = ComplexMatrix::new();
    ///```
    pub fn new() -> Self {
        ComplexMatrix {
            entries: vec![],
            order: 0,
        }
    }

    /// Create a new ```ComplexMatrix``` from a vector of ```(row, col, (real, imag))``` entries.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let entries = vec![(0, 0, (1., 1.)), (1, 1, (1., 1.))];
    /// let mut m = ComplexMatrix::from_entries(entries);
    ///```
    pub fn from_entries(entries: Vec<Entry<T>>) -> Self {
        ComplexMatrix { entries, order: 0 }
    }

    fn set_order(&mut self) {
        for &entry in self.entries.iter() {
            if entry.row + 1 > self.order {
                self.order = entry.row + 1
            }
            if entry.col + 1 > self.order {
                self.order = entry.col + 1
            }
        }
    }

    /// Add or set an element at location ```(row, col)``` with value ```(real, imag)```.
    pub fn add_element(&mut self, new: Entry<T>) {
        if new.value != Complex::<T>::zero() {
            self.entries = self
                .entries
                .iter()
                .copied()
                .filter(|&entry| entry.row != new.row && entry.col != new.col)
                .collect();

            self.entries.push(new);
        }
    }

    /// Add elements correspoding to each triplet ```(row, col, (real, imag))```.
    pub fn add_elements(&mut self, entries: Vec<Entry<T>>) {
        for entry in entries.into_iter() {
            self.add_element(entry)
        }
    }

    ///  Returns the Element-value at ```(row, col)``` if present, or None if not.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let entries = vec![(0, 0, (1., -1.)), (1, 1, (-1., 1.))];
    /// let mut m = ComplexMatrix::from_entries(entries);
    /// assert_eq!(m.get(2,1), None);
    /// assert_eq!(m.get(0,0), Some((1., -1.)));
    /// assert_eq!(m.get(1,1), Some((-1., 1.)));
    ///```
    pub fn get(&self, row: usize, col: usize) -> Option<Entry<T>> {
        self.entries
            .iter()
            .copied()
            .filter(|&entry| (entry.row == row) && (entry.col == col))
            .next()
    }

    ///  Get the order of the matrix.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let entries = vec![(0, 0, (1., -1.)), (1, 1, (-1., 1.))];
    /// let mut m = ComplexMatrix::from_entries(entries);
    /// assert_eq!(m.order(), 2);
    /// m.add_element(3, 3, (2., 2.));
    /// assert_eq!(m.order(), 4);
    ///```
    pub fn order(&mut self) -> usize {
        self.set_order();
        self.order
    }

    /// Solve the system `Ax=b`, where:
    /// * `A` is a complex matrix
    /// * `b` is a complex vector
    /// * `x` is the return value.
    ///
    /// Returns a `Result` containing the vector with ```(real, imag)``` solutions.
    /// Returns an `Err` if unsuccessful.
    ///
    /// This solution use the LU factorization implemented by [sparse21](https://crates.io/crates/sparse21/).
    ///
    ///```rust
    ///     use sparse_complex::ComplexMatrix;
    ///
    ///     let mut A = ComplexMatrix::new();
    ///     A.add_element(0, 0, (1., 1.));
    ///     A.add_element(1, 1, (1., 1.));
    ///
    ///     let b = [(1., 0.), (0., 1.)];
    ///
    ///     let solution = A.solve(&b);
    ///     assert_eq!(solution.unwrap(), vec![(0.5, -0.5), (0.5, 0.5)]);
    ///```
    pub fn solve(&mut self, _b: &mut [Complex<T>]) -> Result<(), &'static str> {
        todo!()
    }
}

impl<T: Float + std::fmt::Display> fmt::Debug for ComplexMatrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::from("ComplexMatrix { \n");
        for entry in self.entries.iter() {
            msg = format!(
                "{}  ({},{}) -> {}\n",
                msg, entry.row, entry.col, entry.value
            )
        }
        write!(f, "{}}}", msg)
    }
}

#[cfg(test)]
mod tests_simple_matrix {
    use super::*;
    const Z1: Complex<f64> = Complex { re: 1., im: -1. };
    const Z2: Complex<f64> = Complex { re: -1., im: 1. };

    #[test]
    fn test_add_element() {
        let mut m = ComplexMatrix::new();
        m.add_element(Entry {
            row: 0,
            col: 0,
            value: Z1,
        });
        m.add_element(Entry {
            row: 1,
            col: 1,
            value: Z2,
        });

        assert_eq!(m.get(0, 0).unwrap().value, Z1);
        assert_eq!(m.get(1, 1).unwrap().value, Z2);
    }

    #[test]
    fn test_add_elements() {
        let mut m = ComplexMatrix::new();
        let entries = vec![Entry::new(0, 0, Z1), Entry::new(1, 1, Z2)];
        m.add_elements(entries);
        assert_eq!(m.get(0, 0).unwrap().value, Z1);
        assert_eq!(m.get(1, 1).unwrap().value, Z2);
    }

    #[test]
    #[ignore = "Solve not implemented"]
    fn test_solve() {
        let mut m = ComplexMatrix::new();
        m.add_element(Entry::new(0, 0, Z1));
        m.add_element(Entry::new(1, 1, Z2));
        let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
        m.solve(&mut b).unwrap();

        let expected = vec![Complex::new(0.5, 0.5), Complex::new(0.5, -0.5)];
        assert_eq!(b, expected);
    }
}
