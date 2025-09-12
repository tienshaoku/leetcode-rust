fn check_if_array_is_sorted_and_rotated(nums: Vec<i32>) -> bool {
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] > nums[(i + 1) % nums.len()] {
            count += 1;
        }
        // if sorted, count should be 1 only
        if count > 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod check_if_array_is_sorted_and_rotated_test {
    use super::*;

    #[test]
    fn check_if_array_is_sorted_and_rotated_test_1() {
        assert_eq!(
            check_if_array_is_sorted_and_rotated(vec![3, 4, 5, 1, 2]),
            true
        );
    }

    #[test]
    fn check_if_array_is_sorted_and_rotated_test_2() {
        assert_eq!(
            check_if_array_is_sorted_and_rotated(vec![2, 1, 3, 4]),
            false
        );
    }

    #[test]
    fn check_if_array_is_sorted_and_rotated_test_3() {
        assert_eq!(check_if_array_is_sorted_and_rotated(vec![1, 2, 3]), true);
    }
}
