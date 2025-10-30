fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut sum = nums[0];
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            sum += nums[i];
        } else {
            max = max.max(sum);
            sum = nums[i];
        }
    }
    max = max.max(sum);
    max
}

#[cfg(test)]
mod max_ascending_sum_test {
    use super::*;

    #[test]
    fn max_ascending_sum_test_1() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    }

    #[test]
    fn max_ascending_sum_test_2() {
        assert_eq!(max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    }

    #[test]
    fn max_ascending_sum_test_3() {
        assert_eq!(max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]), 33);
    }

    #[test]
    fn max_ascending_sum_test_4() {
        assert_eq!(max_ascending_sum(vec![100, 10, 1]), 100);
    }

    #[test]
    fn max_ascending_sum_test_5() {
        assert_eq!(max_ascending_sum(vec![3, 6, 10, 1, 8, 9, 9, 8, 9]), 19);
    }
}
