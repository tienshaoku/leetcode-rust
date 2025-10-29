fn find_numbers_with_even_digits(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in nums {
        let count = (i as f64).log10().floor();
        if count as i32 % 2 != 0 {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod find_numbers_with_even_digits_test {
    use super::*;

    #[test]
    fn find_numbers_with_even_digits_test_1() {
        assert_eq!(
            find_numbers_with_even_digits(vec![12, 345, 2, 9, 6, 7896]),
            2
        );
    }

    #[test]
    fn find_numbers_with_even_digits_test_2() {
        assert_eq!(
            find_numbers_with_even_digits(vec![555, 901, 482, 1771, 99]),
            2
        );
    }

    #[test]
    fn find_numbers_with_even_digits_test_3() {
        assert_eq!(find_numbers_with_even_digits(vec![100000]), 1);
    }
}
