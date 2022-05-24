# sparse_complex

[![crate](https://img.shields.io/crates/v/sparse_complex.svg)](https://crates.io/crates/sparse_complex)
[![documentation](https://docs.rs/sparse_complex/badge.svg)](https://docs.rs/sparse_complex)
[![minimum rustc 1.61](https://img.shields.io/badge/rustc-1.61+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![build status](https://github.com/felipemarkson/sparse_complex/actions/workflows/main.yml/badge.svg)](https://github.com/felipemarkson/sparse_complex/actions)

An simple solver for sparse complex linear systems based on [Eigen::SparseLU](https://eigen.tuxfamily.org/dox/classEigen_1_1SparseLU.html).

## Complex Number representation

We use [num::Complex<T>](https://docs.rs/num/latest/num/struct.Complex.html) to represent complex numbers. See [num](https://docs.rs/num/latest/num/) crate for more information.

## Example

Lets consider the complex linear system bellow:
  
$$
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
$$

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
  
$$
\frac{1}{2}
\begin{bmatrix}
1 + j1 \\
1 - j1
\end{bmatrix}
$$

## Version Compatible
The ```sparse_complex``` crate is tested for `rustc` 1.61 and greater.


## License
MIT License. See [LICENSE](/LICENSE).

_**`sparse_complex` also depends on Eigen v3.4.0 which is licensed under [MPL v2.0](https://www.mozilla.org/en-US/MPL/2.0/). The source code of Eigen can be found on [Eigen's Home Page](https://eigen.tuxfamily.org/).**_
