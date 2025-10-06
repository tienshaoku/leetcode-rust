fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut arr = vec![0; nums.len()];
    for i in (0..nums.len()).rev() {
        let mut now = nums[i];
        if i + 2 < nums.len() {
            if i + 3 < nums.len() && arr[i + 2] < arr[i + 3] {
                now += arr[i + 3];
            } else {
                now += arr[i + 2]
            }
        }
        arr[i] = now;
    }
    arr[0].max(arr[1])
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
