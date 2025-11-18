fn jump_game(nums: Vec<i32>) -> bool {
    let mut farthest = 0;

    for i in 0..nums.len() {
        if i > farthest {
            return false;
        }

        farthest = farthest.max(i + nums[i] as usize);

        if farthest >= nums.len() - 1 {
            return true;
        }
    }
    false
}

fn jump_game_slow(nums: Vec<i32>) -> bool {
    let mut arr = vec![false; nums.len()];
    arr[0] = true;
    let mut i = 0;
    while i < nums.len() && arr[i] {
        let mut current = nums[i];
        if (i + current as usize) >= nums.len() - 1 {
            return true;
        }
        while current > 0 {
            if (i + current as usize) < nums.len() {
                arr[i + current as usize] = true;
            }
            current -= 1;
        }
        i += 1;
    }
    *arr.last().unwrap()
}

#[cfg(test)]
mod jump_game_test {
    use super::*;

    #[test]
    fn jump_game_test_1() {
        assert_eq!(jump_game(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn jump_game_test_2() {
        assert_eq!(jump_game(vec![3, 2, 1, 0, 4]), false);
    }

    #[test]
    fn jump_game_test_3() {
        assert_eq!(jump_game(vec![4, 1, 0, 7, 2, 1]), true);
    }

    #[test]
    fn jump_game_test_4() {
        assert_eq!(jump_game(vec![4, 1, 0, 1, 0, 1]), false);
    }

    #[test]
    fn jump_game_test_5() {
        assert_eq!(jump_game(vec![4]), true);
    }

    #[test]
    fn jump_game_test_6() {
        assert_eq!(jump_game(vec![0, 2, 3]), false);
    }
}
