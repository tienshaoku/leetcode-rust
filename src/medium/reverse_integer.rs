fn reverse_integer(x: i32) -> i32 {
    let mut x = x;
    let mut res: i32 = 0;
    while x != 0 {
        if let Some(v) = res.checked_mul(10).and_then(|r| r.checked_add(x % 10)) {
            res = v;
        } else {
            return 0;
        }
        x /= 10;
    }
    res
}

#[cfg(test)]
mod reverse_integer_test {
    use super::*;

    #[test]
    fn reverse_integer_test_1() {
        assert_eq!(reverse_integer(321), 123);
    }

    #[test]
    fn reverse_integer_test_2() {
        assert_eq!(reverse_integer(-123), -321);
    }

    #[test]
    fn reverse_integer_test_3() {
        assert_eq!(reverse_integer(120), 21);
    }
}
