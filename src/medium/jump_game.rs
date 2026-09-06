fn jump_game(nums: Vec<i32>) -> bool {
    let mut farthest = 0;

    for i in 0..nums.len() {
        if farthest >= i {
            farthest = farthest.max(i + nums[i] as usize);
        }
        if farthest >= nums.len() - 1 {
            return true;
        }
    }
    false
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

    #[test]
    fn jump_game_test_7() {
        assert_eq!(jump_game(vec![2, 5, 0, 0]), true);
    }
}
