fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();

    let length = nums.len();
    let mut res = vec![];
    for i in 0..length.saturating_sub(2) {
        if nums[i] > 0 {
            break;
        }
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = length - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum > 0 {
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                res.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                right -= 1;

                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            }
        }
    }
    res
}

fn three_sum_slow(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;
    use std::collections::HashSet;
    fn two_sum_sorted(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut res = vec![];
        while left < right {
            match (nums[left] + nums[right]).cmp(&target) {
                Ordering::Equal => {
                    res.push(vec![nums[left], nums[right]]);
                    left += 1;
                }
                Ordering::Greater => right -= 1,
                _ => left += 1,
            }
        }
        res
    }

    let mut nums = nums;
    nums.sort();
    let mut set = HashSet::new();
    for i in 0..nums.len() - 1 {
        let arr = two_sum_sorted(&nums[i + 1..], -nums[i]);
        if !arr.is_empty() {
            for a in arr {
                set.insert(vec![nums[i], a[0], a[1]]);
            }
        }
    }
    set.into_iter().collect()
}

#[cfg(test)]
mod three_sum_test {
    use super::*;
    use crate::vector::normalise;

    #[test]
    fn three_sum_test_1() {
        assert_eq!(
            normalise(three_sum(vec![-1, 0, 1, 2, -1, -4])),
            normalise(vec![vec![-1, -1, 2], vec![-1, 0, 1]])
        );
    }

    #[test]
    fn three_sum_test_2() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn three_sum_test_3() {
        assert_eq!(three_sum(vec![0, 0, 0]), [[0, 0, 0]]);
    }

    #[test]
    fn three_sum_test_4() {
        assert_eq!(
            normalise(three_sum(vec![-7, -5, -3, -1, 1, 2, 4])),
            normalise(vec![vec![-3, 1, 2], vec![-5, 1, 4], vec![-3, -1, 4]])
        );
    }

    #[test]
    fn three_sum_test_5() {
        assert_eq!(three_sum(vec![0, 0, 0, 0]), [[0, 0, 0]]);
    }

    #[test]
    fn three_sum_test_6() {
        assert_eq!(
            normalise(three_sum(vec![-100, -70, -60, 110, 120, 130, 160])),
            normalise(vec![vec![-100, -60, 160], vec![-70, -60, 130]])
        );
    }
}
