fn rob_two(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    fn rob(nums: &[i32]) -> i32 {
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

    let one = rob(&nums[..nums.len() - 1]);
    let two = rob(&nums[1..nums.len()]);
    one.max(two)
}

#[cfg(test)]
mod rob_two_test {
    use super::*;

    #[test]
    fn rob_two_test_1() {
        assert_eq!(rob_two(vec![2, 3, 2]), 3);
    }

    #[test]
    fn rob_two_test_2() {
        assert_eq!(rob_two(vec![1, 2, 3]), 3);
    }

    #[test]
    fn rob_two_test_3() {
        assert_eq!(rob_two(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn rob_two_test_4() {
        assert_eq!(rob_two(vec![0]), 0);
    }

    #[test]
    fn rob_two_test_5() {
        assert_eq!(rob_two(vec![2, 2, 4, 11, 8]), 13);
    }

    #[test]
    fn rob_two_test_6() {
        assert_eq!(rob_two(vec![200, 3, 140, 20, 10]), 340);
    }

    #[test]
    fn rob_two_test_7() {
        assert_eq!(rob_two(vec![2, 1, 1, 2]), 3);
    }

    #[test]
    fn rob_two_test_8() {
        assert_eq!(rob_two(vec![2, 1, 1, 2]), 3);
    }

    #[test]
    fn rob_two_test_9() {
        assert_eq!(rob_two(vec![8, 2, 8, 9, 2]), 17);
    }

    #[test]
    fn rob_two_test_10() {
        assert_eq!(rob_two(vec![8, 4, 8, 5, 9, 6, 5, 4, 4, 10]), 34);
    }
}
