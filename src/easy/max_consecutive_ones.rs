fn max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut max = 0;
    for i in nums {
        if i == 0 {
            count = 0;
        } else {
            count += 1;
            max = max.max(count);
        }
    }
    max
}

#[cfg(test)]
mod max_consecutive_ones_test {
    use super::*;

    #[test]
    fn max_consecutive_ones_test_1() {
        assert_eq!(max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }

    #[test]
    fn max_consecutive_ones_test_2() {
        assert_eq!(max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
