fn trailing_zeroes(n: i32) -> i32 {
    let mut sum = 0;
    let mut i = 5;
    while i <= n {
        sum += n / i;
        i *= 5;
    }
    sum
}

#[cfg(test)]
mod trailing_zeroes_test {
    use super::*;

    #[test]
    fn trailing_zeroes_test_1() {
        assert_eq!(trailing_zeroes(20), 4);
    }

    #[test]
    fn trailing_zeroes_test_2() {
        assert_eq!(trailing_zeroes(5), 1);
    }

    #[test]
    fn trailing_zeroes_test_3() {
        assert_eq!(trailing_zeroes(0), 0);
    }

    #[test]
    fn trailing_zeroes_test_4() {
        assert_eq!(trailing_zeroes(3), 0);
    }

    #[test]
    fn trailing_zeroes_test_5() {
        assert_eq!(trailing_zeroes(30), 7);
    }

    #[test]
    fn trailing_zeroes_test_6() {
        assert_eq!(trailing_zeroes(3125), 781);
    }
}
