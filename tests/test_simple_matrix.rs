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

    #[test]
    fn test_full_simple_matrix() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, (5., 3.));
        m.add_element(1, 1, (1., -9.));
        m.add_element(0, 1, (-33., 0.));
        m.add_element(1, 0, (0., -47.));



        let solution = m.solve(&[(13.4, 7.), (3.2, -7.)]);
        assert_eq!(solution.unwrap(), vec![(0.21852826260018543, 0.10986007256547237), (-0.3829375425665308, -0.1756095408900631)]);
    }

    #[test]
    fn test_only_imag_simple_matrix() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, (0., 3.));
        m.add_element(0, 1, (0., -33.));
        m.add_element(1, 0, (0., -1.));
        m.add_element(1, 1, (0., 9.));


        let solution = m.solve(&[(0., 3.), (0., 6.)]);
        assert_eq!(solution.unwrap(), vec![(-37.5, 0.), (-3.5, 0.)]);
    }

    #[test]
    fn test_only_real_simple_matrix() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, (3., 0.));
        m.add_element(0, 1, (-7., 0.));
        m.add_element(1, 0, (-1., 0.));
        m.add_element(1, 1, (9., 0.));


        let solution = m.solve(&[(3., 0.), (6., 0.)]);
        assert_eq!(solution.unwrap(), vec![(3.4500000000000006, 0.), (1.05, 0.)]);
    }

    #[test]
    fn test_part_real_part_imag_simple_matrix() {
        let mut m = ComplexMatrix::new();
        m.add_element(0, 0, (3., 0.));
        m.add_element(0, 1, (0., 1.));
        m.add_element(1, 0, (0., -4.));
        m.add_element(1, 1, (1., 0.));


        let solution = m.solve(&[(0., 3.), (6., 0.)]);
        assert_eq!(solution.unwrap(), vec![(0., 3.), (-6., 0.)]);
    }
}
