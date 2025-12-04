fn count_partitions(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut right = sum;
    let mut res = 0;
    for i in 0..nums.len() - 1 {
        right -= nums[i];
        if (sum - 2 * right) % 2 == 0 {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod count_partitions_test {
    use super::*;

    #[test]
    fn count_partitions_test_1() {
        assert_eq!(count_partitions(vec![10, 10, 3, 7, 6]), 4);
    }

    #[test]
    fn count_partitions_test_2() {
        assert_eq!(count_partitions(vec![1, 2, 2]), 0);
    }

    #[test]
    fn count_partitions_test_3() {
        assert_eq!(count_partitions(vec![2, 4, 6, 8]), 3);
    }

    #[test]
    fn count_partitions_test_4() {
        assert_eq!(count_partitions(vec![2]), 0);
    }
}
