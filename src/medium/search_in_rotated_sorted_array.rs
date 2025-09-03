fn search_in_rotated_sorted_array(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        }

        if nums[left] <= nums[mid] {
            if nums[left] <= target && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid] < target && target <= nums[right] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }
    -1
}

// a flashy alternative:
// 1. find_min_in_rotated_sorted_array() to find out min's index
// 2. binary_search() the right section
fn search_in_rotated_sorted_array_flashy_but_too_complex(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;

    if nums.len() == 1 {
        return if nums[0] == target { 0 } else { -1 };
    }

    let min_idx = find_min_idx(&nums);
    if target == nums[min_idx] {
        return min_idx as i32;
    }
    let (mut left, mut right) = (0, 0);
    if min_idx == 0 {
        right = nums.len() - 1;
    } else {
        match target.cmp(&nums[nums.len() - 1]) {
            Ordering::Equal => {
                return nums.len() as i32 - 1;
            }
            Ordering::Greater => {
                right = min_idx - 1;
            }
            _ => {
                (left, right) = (min_idx, nums.len() - 1);
            }
        }
    }

    let result = binary_search(nums[left..=right].to_vec(), target);
    if result == -1 {
        -1
    } else {
        left as i32 + result
    }
}

fn find_min_idx(nums: &Vec<i32>) -> usize {
    let mut min = i32::MAX;
    let mut idx = 0;
    let (mut left, mut right) = (0, nums.len() - 1);
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] < min {
            idx = mid;
            min = nums[mid];
        }

        if nums[left] <= nums[right] {
            if nums[left] < min {
                idx = left;
                min = nums[left];
            }
            break;
        }

        if nums[mid] > nums[right] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    idx
}

fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;

    let (mut left, mut right) = (0, nums.len());
    while left < right {
        let middle = left + (right - left) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Equal => return middle as i32,
            Ordering::Greater => right = middle,
            Ordering::Less => left = middle + 1,
        }
    }
    -1
}

#[cfg(test)]
mod search_in_rotated_sorted_array_test {
    use super::*;

    #[test]
    fn search_in_rotated_sorted_array_test_1() {
        assert_eq!(
            search_in_rotated_sorted_array(vec![4, 5, 6, 7, 0, 1, 2], 0),
            4
        );
    }

    #[test]
    fn search_in_rotated_sorted_array_test_2() {
        assert_eq!(
            search_in_rotated_sorted_array(vec![4, 5, 6, 7, 0, 1, 2], 3),
            -1
        );
    }

    #[test]
    fn search_in_rotated_sorted_array_test_3() {
        assert_eq!(search_in_rotated_sorted_array(vec![1], 0), -1);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_4() {
        assert_eq!(search_in_rotated_sorted_array(vec![1, 3], 0), -1);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_5() {
        assert_eq!(search_in_rotated_sorted_array(vec![1, 3], 3), 1);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_6() {
        assert_eq!(search_in_rotated_sorted_array(vec![3, 5, 1], 3), 0);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_7() {
        assert_eq!(search_in_rotated_sorted_array(vec![3, 1], 1), 1);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_8() {
        assert_eq!(search_in_rotated_sorted_array(vec![1, 3, 5], 2), -1);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_9() {
        assert_eq!(search_in_rotated_sorted_array(vec![3, 5, 1], 1), 2);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_10() {
        assert_eq!(
            search_in_rotated_sorted_array(vec![4, 5, 6, 7, 8, 1, 2, 3], 8),
            4
        );
    }

    #[test]
    fn search_in_rotated_sorted_array_test_11() {
        assert_eq!(search_in_rotated_sorted_array(vec![5, 1, 2, 3, 4], 1), 1);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_12() {
        assert_eq!(search_in_rotated_sorted_array(vec![3, 1], 3), 0);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_13() {
        assert_eq!(search_in_rotated_sorted_array(vec![5, 1, 3], 3), 2);
    }

    #[test]
    fn search_in_rotated_sorted_array_test_14() {
        assert_eq!(
            search_in_rotated_sorted_array(vec![4, 5, 6, 7, 0, 1, 2], 1),
            5
        );
    }
}
