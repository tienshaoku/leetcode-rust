fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort();
    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }
    false
}

fn contains_duplicate_slow(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    for i in nums {
        if !set.insert(i) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod contains_duplicate_test {
    use super::*;

    #[test]
    fn contains_duplicate_test_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn contains_duplicate_test_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn contains_duplicate_test_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }
}
