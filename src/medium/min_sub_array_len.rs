fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let (mut left, mut right) = (0, 1);
    let mut sum = nums[0];
    while left < right && right <= nums.len() {
        while sum < target && right < nums.len() {
            if nums[right] >= target {
                return 1;
            }
            sum += nums[right];
            right += 1;
        }
        if sum >= target {
            min = min.min((right - left) as i32);
        }
        sum -= nums[left];
        left += 1;
    }
    if min == i32::MAX {
        0
    } else {
        min
    }
}

#[cfg(test)]
mod min_sub_array_len_test {
    use super::*;

    #[test]
    fn min_sub_array_len_test_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn min_sub_array_len_test_2() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn min_sub_array_len_test_3() {
        assert_eq!(min_sub_array_len(5, vec![1, 2, 2, 1, 4, 0]), 2);
    }

    #[test]
    fn min_sub_array_len_test_4() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn min_sub_array_len_test_5() {
        assert_eq!(min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn min_sub_array_len_test_6() {
        assert_eq!(min_sub_array_len(6, vec![10, 2, 3]), 1);
    }

    #[test]
    fn min_sub_array_len_test_7() {
        assert_eq!(
            min_sub_array_len(15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8]),
            2
        );
    }

    #[test]
    fn min_sub_array_len_test_8() {
        assert_eq!(
            min_sub_array_len(213, vec![12, 28, 83, 4, 25, 26, 25, 2, 25, 25, 25, 12]),
            8
        );
    }
}
