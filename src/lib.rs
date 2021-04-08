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

use sparse21::Matrix;
use std::fmt;
mod tools;

type Entry = (usize, usize, (f64, f64));

/// The complex matrix struct
pub struct ComplexMatrix {
    primitive: Matrix,
    entries: Vec<Entry>,
    builded: bool,
    order: usize,
}

impl ComplexMatrix {
    /// Create a new, initially empty ```ComplexMatrix```
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let mut m = ComplexMatrix::new();
    ///```
    pub fn new() -> Self {
        ComplexMatrix {
            primitive: Matrix::new(),
            entries: vec![],
            builded: false,
            order: 0,
        }
    }

    /// Create a new ```ComplexMatrix``` from a vector of ```(row, col, (real, imag))``` entries.
    ///```rust
    /// use sparse_complex::ComplexMatrix;
    /// let entries = vec![(0, 0, (1., 1.)), (1, 1, (1., 1.))];
    /// let mut m = ComplexMatrix::from_entries(entries);
    ///```
    pub fn from_entries(entries: Vec<Entry>) -> Self {
        let mut result = ComplexMatrix {
            primitive: Matrix::new(),
            entries: entries.to_vec(),
            builded: false,
            order: 0,
        };
        result.build_primitive();
        result
    }

    fn set_in_primitive(&mut self, entry: &Entry) {
        let row = entry.0;
        let col = entry.1;
        let (real, imag) = entry.2;

        const MIN_POS_VALUE:f64 = std::f64::MIN_POSITIVE;

        if real >= MIN_POS_VALUE || real <= -MIN_POS_VALUE{
            self.primitive.add_element(row, col, real);
            self.primitive
            .add_element(row + self.order, col + self.order, real);

        }

        if imag >= MIN_POS_VALUE || imag <= -MIN_POS_VALUE{
            self.primitive.add_element(row, col + self.order, -imag);
            self.primitive.add_element(row + self.order, col, imag);

        }

    }

    fn set_order(&mut self) {
        for &(row_m, col_m, _) in self.entries.iter() {
            if row_m + 1 > self.order {
                self.order = row_m + 1
            }
            if col_m + 1 > self.order {
                self.order = col_m + 1
            }
        }
    }

    fn build_primitive(&mut self) {
        self.set_order();
        for entry in self.entries.clone() {
            self.set_in_primitive(&entry);
        }
        self.builded = true;
    }

    /// Add or set an element at location ```(row, col)``` with value ```(real, imag)```.
    pub fn add_element(&mut self, row: usize, col: usize, value: (f64, f64)) {
        if value != (0., 0.) {
            self.entries = self
                .entries
                .iter()
                .copied()
                .filter(|&entry| entry != (row, col, value))
                .collect();

            self.entries.push((row, col, value));
            self.builded = false;
        }
    }

    /// Add elements correspoding to each triplet ```(row, col, (real, imag))```.
    pub fn add_elements(&mut self, entries: Vec<Entry>) {
        for &(row, col, value) in entries.iter() {
            self.add_element(row, col, value)
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
    pub fn get(&self, row: usize, col: usize) -> Option<(f64, f64)> {
        let option = self
            .entries
            .iter()
            .copied()
            .filter(|&(row_a, col_a, _)| row_a == row && col_a == col)
            .next();

        match option {
            Some((_, _, value)) => Some(value),
            None => None,
        }
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
        if self.builded {
            self.order
        } else {
            self.build_primitive();
            self.order
        }
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
    pub fn solve(&mut self, b: &[(f64, f64)]) -> Result<Vec<(f64, f64)>, &'static str> {
        if !self.builded {
            self.build_primitive();
        }

        let b_primitive: Vec<f64> = tools::complex_to_primitive(b);
        let primitive_result = self.primitive.solve(b_primitive)?;
        tools::primitive_to_complex(&primitive_result)
    }
}

impl Clone for ComplexMatrix {
    fn clone(&self) -> Self {
        ComplexMatrix::from_entries(self.entries.clone())
    }
}

impl PartialEq for ComplexMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl fmt::Debug for ComplexMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::from("ComplexMatrix { \n");
        for (row, col, (real, imag)) in self.entries.iter().copied() {
            if imag < 0. {
                msg = format!("{}  ({},{}) -> {} - j{}\n", msg, row, col, real, -imag)
            } else {
                msg = format!("{}  ({},{}) -> {} + j{}\n", msg, row, col, real, imag)
            }
        }
        write!(f, "{}}}", msg)
    }
}

impl Default for ComplexMatrix {
    /// Returns a 2x2 identity matrix
    fn default() -> Self {
        ComplexMatrix::from_entries(vec![(0, 0, (1., 0.)), (1, 1, (1., 0.))])
    }
}

#[cfg(test)]
mod tests_simple_matrix {
    use super::*;
    const Z1: (f64, f64) = (1., -1.);
    const Z2: (f64, f64) = (-1., 1.);

    fn verify_simple_matrix(m: ComplexMatrix) {
        //Uper-left
        assert_eq!(Z1.0, m.primitive.get(0, 0).unwrap());
        assert!(m.primitive.get(0, 1).is_none());
        assert!(m.primitive.get(1, 0).is_none());
        assert_eq!(Z2.0, m.primitive.get(1, 1).unwrap());

        //Uper-right
        assert_eq!(-Z1.1, m.primitive.get(0, 2).unwrap());
        assert_eq!(-Z2.1, m.primitive.get(1, 3).unwrap());
        assert!(m.primitive.get(0, 3).is_none());
        assert!(m.primitive.get(1, 2).is_none());

        //Lower-left
        assert_eq!(Z1.1, m.primitive.get(2, 0).unwrap());
        assert_eq!(Z2.1, m.primitive.get(3, 1).unwrap());
        assert!(m.primitive.get(2, 1).is_none());
        assert!(m.primitive.get(3, 0).is_none());

        //Lower-right
        assert_eq!(Z1.0, m.primitive.get(2, 2).unwrap());
        assert!(m.primitive.get(2, 3).is_none());
        assert!(m.primitive.get(3, 2).is_none());
        assert_eq!(Z2.0, m.primitive.get(3, 3).unwrap());
    }

    #[test]
    fn test_add_element() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, Z1);
        m.add_element(1, 1, Z2);
        m.build_primitive();
        verify_simple_matrix(m);
    }

    #[test]
    fn test_add_elements() {
        let mut m = ComplexMatrix::new();
        let entries = vec![(0, 0, Z1), (1, 1, Z2)];
        m.add_elements(entries);
        m.build_primitive();

        verify_simple_matrix(m);
    }

    #[test]
    fn test_solve() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, Z1);
        m.add_element(1, 1, Z2);
        let solution = m.solve(&[(1., 0.), (0., 1.)]);
        assert_eq!(solution.unwrap(), vec![(0.5, 0.5), (0.5, -0.5)]);
    }
}

#[cfg(test)]
mod tests_std_traits {
    use super::*;
    const Z1: (f64, f64) = (1., 6.);
    const Z2: (f64, f64) = (3., -1.);

    #[test]
    fn test_debug() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, Z1);
        m.add_element(1, 1, Z2);

        let mut msg = String::from("ComplexMatrix { \n");

        msg = format!("{}  (0,0) -> {} + j{}\n", msg, Z1.0, Z1.1);
        msg = format!("{}  (1,1) -> {} - j{}\n", msg, Z2.0, -Z2.1);
        msg = format!("{}}}", msg);

        assert_eq!(msg, format!("{:?}", m));
    }

    #[test]
    fn test_partial_eq() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, Z1);
        m.add_element(1, 1, Z2);

        let mut a = ComplexMatrix::new();
        a.add_element(0, 0, Z1);
        a.add_element(1, 1, Z2);

        assert!(a == m)
    }

    #[test]
    fn test_default() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, (1., 0.));
        m.add_element(1, 1, (1., 0.));

        let m_default = ComplexMatrix::default();

        assert_eq!(m, m_default)
    }
}
