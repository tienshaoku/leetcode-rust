fn max_subarray(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut sum = nums[0];
    for i in 1..nums.len() {
        if sum < 0 {
            sum = nums[i];
        } else {
            if sum + nums[i] > 0 {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
        }
        max = max.max(sum);
    }
    max
}

#[cfg(test)]
mod max_subarray_test {
    use super::*;

    #[test]
    fn max_subarray_test_1() {
        assert_eq!(max_subarray(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn max_subarray_test_2() {
        assert_eq!(max_subarray(vec![1]), 1);
    }

    #[test]
    fn max_subarray_test_3() {
        assert_eq!(max_subarray(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn max_subarray_test_4() {
        assert_eq!(max_subarray(vec![5, 4, -1, -1, -1, -1, 7, 8]), 20);
    }

    #[test]
    fn max_subarray_test_5() {
        assert_eq!(max_subarray(vec![-1, -1, -1, -1, -2]), -1);
    }
}
