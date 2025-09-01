fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    use std::cmp::Ordering;

    // 1. use len() instead of len() - 1
    let (mut left, mut right) = (0, nums.len());
    while left < right {
        // overflow prevention
        let middle = left + (right - left) / 2;
        match nums[middle].cmp(&target) {
            Ordering::Equal => return middle as i32,
            Ordering::Greater => right = middle,
            // 2. only increment left (and don't sub 1 from right)
            Ordering::Less => left = middle + 1,
        }
    }
    -1
    // 1 + 2: use exclusive right bound s.t.
    // a. right - left == length
    // b. (left + right) / 2 won't fall into infinite loop
    //    cuz essentially right is exclusive, i.e. 1 larger than its real index
}

fn binary_search_redundant_comparison(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len());
    let mut mid = (right + left) / 2;
    loop {
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            right = mid;
        } else {
            left = mid;
        }

        let new_mid = (right + left) / 2;
        if mid == new_mid {
            return -1;
        }
        mid = new_mid;
    }
}

#[cfg(test)]
mod binary_search_test {
    use super::*;

    #[test]
    fn binary_search_test_1() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn binary_search_test_2() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn binary_search_test_3() {
        assert_eq!(binary_search(vec![2], 2), 0);
    }

    #[test]
    fn binary_search_test_4() {
        assert_eq!(binary_search(vec![3], 2), -1);
    }

    #[test]
    fn binary_search_test_5() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], -1), -0);
    }

    #[test]
    fn binary_search_test_6() {
        assert_eq!(binary_search(vec![2, 5], 5), 1);
    }
}
