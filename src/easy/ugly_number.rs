fn ugly_number(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut n = n;
    while n % 2 == 0 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    while n % 5 == 0 {
        n /= 5;
    }
    n == 1
}

#[cfg(test)]
mod ugly_number_test {
    use super::*;

    #[test]
    fn ugly_number_test_1() {
        assert_eq!(ugly_number(6), true);
    }

    #[test]
    fn ugly_number_test_2() {
        assert_eq!(ugly_number(1), true);
    }

    #[test]
    fn ugly_number_test_3() {
        assert_eq!(ugly_number(14), false);
    }

    #[test]
    fn ugly_number_test_4() {
        assert_eq!(ugly_number(-1), false);
    }

    #[test]
    fn ugly_number_test_5() {
        assert_eq!(ugly_number(0), false);
    }
}
