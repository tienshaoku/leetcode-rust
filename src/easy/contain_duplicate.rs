fn contain_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    for i in nums {
        if !set.insert(i) {
            return true;
        }
    }
    false
}

fn contain_duplicate_slow(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    for i in nums {
        if set.contains(&i) {
            return true;
        }
        set.insert(i);
    }
    false
}

#[cfg(test)]
mod contain_duplicate_test {
    use super::*;

    #[test]
    fn contain_duplicate_test_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contain_duplicate(nums), true);
    }

    #[test]
    fn contain_duplicate_test_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contain_duplicate(nums), false);
    }

    #[test]
    fn contain_duplicate_test_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contain_duplicate(nums), true);
    }
}
