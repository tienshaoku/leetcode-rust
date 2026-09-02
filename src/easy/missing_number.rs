fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    (n + 1) * n / 2 - nums.iter().sum::<i32>()
}

#[cfg(test)]
mod missing_number_test {
    use super::*;

    #[test]
    fn missing_number_test_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn missing_number_test_2() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn missing_number_test_3() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn missing_number_test_4() {
        assert_eq!(missing_number(vec![0]), 1);
    }
}
