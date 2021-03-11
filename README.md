# sparse_complex

[![crate](https://img.shields.io/crates/v/sparse_complex.svg)](https://crates.io/crates/sparse_complex)
[![documentation](https://docs.rs/sparse_complex/badge.svg)](https://docs.rs/sparse_complex)
[![minimum rustc 1.50](https://img.shields.io/badge/rustc-1.50+-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![build status](https://github.com/felipemarkson/sparse_complex/actions/workflows/main.yml/badge.svg)](https://github.com/felipemarkson/sparse_complex/actions)

An abstraction layer for [sparse21](https://crates.io/crates/sparse21/) that adds support for complex sparse matrices.

## Complex Number representation

In this implementation, a complex number is represented as a tuple of ```f64```. 
Where the first element is the real part and the second is the imaginary part, as shown bellow:

```rust
let complex_number: (f64, f64) = (1.0 , 1.0) // 1 + j1
```
The use of ```f64``` is a limitation of [sparse21](https://crates.io/crates/sparse21/).


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sparse_complex = "0.1"
```


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


## License
MIT License. See [LICENSE](/LICENSE).