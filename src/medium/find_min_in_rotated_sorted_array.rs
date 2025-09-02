fn find_min(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    // len() - 1 instead of len() cuz we do need to access items
    let (mut left, mut right) = (0, nums.len() - 1);
    // check == for we always +1 or -1; range values can be the same
    while left <= right {
        let mid = left + (right - left) / 2;
        min = min.min(nums[mid]);

        if nums[left] <= nums[right] {
            min = min.min(nums[left]);
            break;
        }

        // always +1 or -1 can avoid checking if mid is stale
        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    min
}

#[cfg(test)]
mod find_min_test {
    use super::*;

    #[test]
    fn find_min_test_1() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn find_min_test_2() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn find_min_test_3() {
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn find_min_test_4() {
        assert_eq!(find_min(vec![11]), 11);
    }

    #[test]
    fn find_min_test_5() {
        assert_eq!(find_min(vec![11, 1, 2, 3, 7, 9]), 1);
    }

    #[test]
    fn find_min_test_6() {
        assert_eq!(find_min(vec![2, 1]), 1);
    }

    #[test]
    fn find_min_test_7() {
        assert_eq!(find_min(vec![3, 1, 2]), 1);
    }
}
