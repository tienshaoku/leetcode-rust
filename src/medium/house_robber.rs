fn rob(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;

    let length = nums.len();
    match length {
        1 => return nums[0],
        2 => return nums[0].max(nums[1]),
        _ => (),
    }

    let mut res = vec![0; length];
    for i in 0..length {
        res[i] = match i.cmp(&2) {
            Ordering::Less => nums[i],
            Ordering::Equal => nums[i] + res[0],
            Ordering::Greater => nums[i] + res[i - 2].max(res[i - 3]),
        };
    }
    res[length - 1].max(res[length - 2])
}

#[cfg(test)]
mod rob_test {
    use super::*;

    #[test]
    fn rob_test_1() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn rob_test_2() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn rob_test_3() {
        assert_eq!(rob(vec![2, 1, 1, 2]), 4);
    }

    #[test]
    fn rob_test_4() {
        assert_eq!(rob(vec![2, 1, 1, 2, 4, 5, 5, 9]), 18);
    }

    #[test]
    fn rob_test_5() {
        assert_eq!(rob(vec![0]), 0);
    }
}
