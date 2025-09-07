fn sum_zero(n: i32) -> Vec<i32> {
    let mut copied = n;
    let mut res = vec![];
    let mut i = 1;
    while copied >= 2 {
        res.push(i);
        res.push(-i);
        i += 1;
        copied -= 2;
    }
    if copied == 1 {
        res.push(0);
    }
    res
}

#[cfg(test)]
mod sum_zero_test {
    use super::*;

    #[test]
    fn sum_zero_test_1() {
        assert_eq!(sum_zero(3), vec![1, -1, 0]);
    }

    #[test]
    fn sum_zero_test_2() {
        assert_eq!(sum_zero(1), vec![0]);
    }

    #[test]
    fn sum_zero_test_3() {
        assert_eq!(sum_zero(5), vec![1, -1, 2, -2, 0]);
    }
}
