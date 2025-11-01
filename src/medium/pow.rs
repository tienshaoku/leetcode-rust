fn pow(x: f64, n: i32) -> f64 {
    x.powi(n)
}

#[cfg(test)]
mod pow_test {
    use super::*;

    #[test]
    fn pow_test_1() {
        assert_eq!(pow(2.0, 10), 1024.0);
    }

    #[test]
    fn pow_test_2() {
        assert_eq!(pow(2.1, 3), 9.261000000000001);
    }

    #[test]
    fn pow_test_3() {
        assert_eq!(pow(2.0, -2), 0.25);
    }
}
