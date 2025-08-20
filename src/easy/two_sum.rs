// hashmap.get(key) -> value
// hashmap cannot efficiently get the key by value, since hashmap is optimised for key-based lookups

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    let mut res = vec![];
    for i in 0..nums.len() {
        match map.get(&(target - nums[i])) {
            Some(v) => {
                res.push(*v as i32);
                res.push(i as i32);
                break;
            }
            None => map.insert(nums[i], i),
        };
    }
    res
}

#[cfg(test)]
mod two_sum_test {
    use super::*;

    #[test]
    fn two_sum_test_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert!(result == vec![0, 1] || result == vec![1, 0]);
    }

    #[test]
    fn two_sum_test_2() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert!(result == vec![0, 1] || result == vec![1, 0]);
    }

    #[test]
    fn two_sum_test_3() {
        let nums = vec![3, 2, 7, 9, 0, -1];
        let target = 6;
        let result = two_sum(nums, target);
        assert!(result == vec![5, 2] || result == vec![2, 5]);
    }

    #[test]
    fn two_sum_test_4() {
        let nums = vec![0, 3, -3, 4, -1];
        let target = -1;
        let result = two_sum(nums, target);
        assert!(result == vec![0, 4] || result == vec![4, 0]);
    }
}
