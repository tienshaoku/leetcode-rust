fn count_bits(n: i32) -> Vec<i32> {
    let mut res = vec![0; n as usize + 1];
    for i in 1..n as usize + 1 {
        res[i] = res[i >> 1] + (i & 1) as i32;
    }
    res
}

#[cfg(test)]
mod count_bits_test {
    use super::*;

    #[test]
    fn count_bits_test_1() {
        assert_eq!(count_bits(2), [0, 1, 1]);
    }

    #[test]
    fn count_bits_test_2() {
        assert_eq!(count_bits(5), [0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn count_bits_test_3() {
        assert_eq!(count_bits(12), [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2]);
    }
}
