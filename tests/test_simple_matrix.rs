#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use sparse_complex::*;
    #[test]
    //#[ignore = "Solve not implemented"]
    fn test_simple_matrix() {
        let mut m = ComplexMatrix::<f64>::new();
        let mut b = vec![Complex::new(1., 0.), Complex::new(0., 1.)];
        m.add_element(0, 0, Complex::new(1., 1.));
        m.add_element(1, 1, Complex::new(1., 1.));
        m.solve(&mut b).unwrap();

        let expected = vec![Complex::new(0.5, -0.5), Complex::new(0.5, 0.5)];
        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    // #[ignore = "Solve not implemented"]
    fn test_full_simple_matrix() {
        let mut m = ComplexMatrix::<f64>::new();
        m.add_element(0, 0, Complex::new(5., 3.));
        m.add_element(1, 1, Complex::new(1., -9.));
        m.add_element(0, 1, Complex::new(-33., 0.));
        m.add_element(1, 0, Complex::new(0., -47.));

        let mut b = vec![Complex::new(13.4, 7.), Complex::new(3.2, -7.)];
        let expected = vec![
            Complex::new(0.21852826260018543, 0.10986007256547237),
            Complex::new(-0.3829375425665308, -0.1756095408900631),
        ];

        m.solve(&mut b).unwrap();

        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_only_imag_simple_matrix() {
        let mut m = ComplexMatrix::<f64>::new();
        m.add_element(0, 0, Complex::new(0., 3.));
        m.add_element(0, 1, Complex::new(0., -33.));
        m.add_element(1, 0, Complex::new(0., -1.));
        m.add_element(1, 1, Complex::new(0., 9.));

        let mut b = vec![Complex::new(0., 3.), Complex::new(0., 6.)];

        m.solve(&mut b).unwrap();

        let expected = vec![Complex::new(-37.5, 0.), Complex::new(-3.5, 0.)];
        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_only_real_simple_matrix() {
        let mut m = ComplexMatrix::<f64>::new();
        m.add_element(0, 0, Complex::new(3., 0.));
        m.add_element(0, 1, Complex::new(-7., 0.));
        m.add_element(1, 0, Complex::new(-1., 0.));
        m.add_element(1, 1, Complex::new(9., 0.));

        let mut b = vec![Complex::new(3., 0.), Complex::new(6., 0.)];

        m.solve(&mut b).unwrap();

        let expected = vec![Complex::new(3.4500000000000006, 0.), Complex::new(1.05, 0.)];
        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_part_real_part_imag_simple_matrix() {
        let mut m = ComplexMatrix::<f64>::new();
        m.add_element(0, 0, Complex::new(3., 0.));
        m.add_element(0, 1, Complex::new(0., 1.));
        m.add_element(1, 0, Complex::new(0., -4.));
        m.add_element(1, 1, Complex::new(1., 0.));

        let mut b = vec![Complex::new(0., 3.), Complex::new(6., 0.)];

        m.solve(&mut b).unwrap();

        let expected = vec![Complex::new(0., 3.), Complex::new(-6., 0.)];
        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_b_imag_is_zero() {
        let mut m = ComplexMatrix::<f64>::new();
        m.add_element(0, 0, Complex::new(3., 0.));
        m.add_element(0, 1, Complex::new(0., 1.));
        m.add_element(1, 0, Complex::new(0., -4.));
        m.add_element(1, 1, Complex::new(1., 0.));

        let mut b = vec![Complex::new(1., 0.), Complex::new(5., 0.)];

        m.solve(&mut b).unwrap();
        let expected = vec![Complex::new(-1., 5.), Complex::new(-15., -4.)];
        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_b_real_is_zero() {
        let mut m = ComplexMatrix::<f64>::new();
        m.add_element(0, 0, Complex::new(3., 0.));
        m.add_element(0, 1, Complex::new(0., 1.));
        m.add_element(1, 0, Complex::new(0., -4.));
        m.add_element(1, 1, Complex::new(1., 0.));

        let mut b = vec![Complex::new(0., 1.), Complex::new(0., 5.)];

        m.solve(&mut b).unwrap();

        let expected = vec![Complex::new(-5., -1.), Complex::new(4., -15.)];
        for (i, e) in b.iter().enumerate() {
            assert_abs_diff_eq!(e.re, &expected[i].re, epsilon = 1e-6);
            assert_abs_diff_eq!(e.im, &expected[i].im, epsilon = 1e-6);
        }
    }
}
