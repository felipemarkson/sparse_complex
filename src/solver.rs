use num::complex::{Complex32, Complex64};



extern "C" {
    pub fn solve_cpp(
        a_matrix: *const Complex64,
        rows: *const usize,
        cols: *const usize,
        n_value: usize,
        b: *mut Complex64,
        size: usize,
    );
    pub fn solve_cpp32(
        a_matrix: *const Complex32,
        rows: *const usize,
        cols: *const usize,
        n_value: usize,
        b: *mut Complex32,
        size: usize,
    );
}