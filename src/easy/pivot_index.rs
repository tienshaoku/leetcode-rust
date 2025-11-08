fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut right: i32 = nums.iter().sum();
    let mut left = 0;
    for i in 0..nums.len() {
        if left == (right - left - nums[i]) {
            return i as i32;
        }
        left += nums[i];
    }
    -1
}

#[cfg(test)]
mod pivot_index_test {
    use super::*;

    #[test]
    fn pivot_index_test_1() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn pivot_index_test_2() {
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn pivot_index_test_3() {
        assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    }
}
