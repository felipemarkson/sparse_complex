#[cfg(test)]
mod tests {
    use sparse_complex::*;
    #[test]
    fn test_simple_matrix() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, (1., 1.));
        m.add_element(1, 1, (1., 1.));
        let solution = m.solve(&[(1., 0.), (0., 1.)]);
        assert_eq!(solution.unwrap(), vec![(0.5, -0.5), (0.5, 0.5)]);
    }
}
