pub fn complex_to_primitive(complex: &[(f64, f64)]) -> Vec<f64> {
    let (real, imag): (Vec<f64>, Vec<f64>) = complex.iter().copied().unzip();
    [real, imag].concat()
}

pub fn primitive_to_complex(primitive: &[f64]) -> Result<Vec<(f64, f64)>, &'static str> {
    let len = primitive.len();
    if len % 2 != 0 {
        return Err("Lenght error");
    }

    Ok(primitive
        .iter()
        .copied()
        .take(len / 2)
        .zip(primitive.iter().copied().skip(len / 2))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_to_primitive() {
        let result = complex_to_primitive(&[(1., -1.), (-2., 2.), (3., -3.)]);

        assert_eq!(vec![1., -2., 3., -1., 2., -3.], result)
    }

    #[test]
    fn test_primitive_to_complex() {
        let result = primitive_to_complex(&[-1., 2., 1., 3.]);
        assert_eq!(vec![(-1., 1.), (2., 3.)], result.unwrap())
    }

    #[test]
    fn test_primitive_to_complex_err() {
        match primitive_to_complex(&[-1., 2., 1.]) {
            Ok(_) => assert!(false),
            Err(msg) => assert_eq!("Lenght error", msg),
        }
    }
}
