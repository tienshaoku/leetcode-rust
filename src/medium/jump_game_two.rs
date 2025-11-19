fn jump_game_two(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }

    let mut jumps: i32 = 0;
    let mut farthest = 0;
    let mut current_end = 0;

    for i in 0..nums.len() - 1 {
        farthest = farthest.max(i + nums[i] as usize);

        if i == current_end {
            jumps += 1;
            current_end = farthest;

            if current_end >= nums.len() - 1 {
                break;
            }
        }
    }
    jumps
}

fn jump_game_two_slow(nums: Vec<i32>) -> i32 {
    let mut arr = vec![0 as i32; nums.len()];
    for i in (0..nums.len() - 1).rev() {
        let mut min = 100000;
        for j in 1..=nums[i] {
            let idx = i + j as usize;
            if idx >= nums.len() {
                min = min.min(arr[i] + 1);
            } else {
                min = min.min(arr[idx] + 1);
            }
        }
        arr[i] = min;
    }
    arr[0]
}

#[cfg(test)]
mod jump_game_two_test {
    use super::*;

    #[test]
    fn jump_game_two_test_1() {
        assert_eq!(jump_game_two(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn jump_game_two_test_2() {
        assert_eq!(jump_game_two(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn jump_game_two_test_3() {
        assert_eq!(jump_game_two(vec![3, 1, 0, 1, 2, 1]), 3);
    }

    #[test]
    fn jump_game_two_test_4() {
        assert_eq!(jump_game_two(vec![4]), 0);
    }
}
