fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut sum = nums[0];
    for i in 1..nums.len() {
        sum = if sum > 0 { sum + nums[i] } else { nums[i] };
        max = max.max(sum);
    }
    max
}

#[cfg(test)]
mod max_sub_array_test {
    use super::*;

    #[test]
    fn max_sub_array_test_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    #[test]
    fn max_sub_array_test_2() {
        assert_eq!(max_sub_array(vec![1]), 1);
    }

    #[test]
    fn max_sub_array_test_3() {
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[test]
    fn max_sub_array_test_4() {
        assert_eq!(max_sub_array(vec![-1]), -1);
    }

    #[test]
    fn max_sub_array_test_5() {
        assert_eq!(max_sub_array(vec![-4, -2]), -2);
    }

    #[test]
    fn max_sub_array_test_6() {
        assert_eq!(max_sub_array(vec![-1, 1, 2, 1]), 4);
    }
}
