fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    for i in (0..nums.len()).rev() {
        if nums[i] == val {
            nums.remove(i);
        }
    }
    nums.len() as i32
}

#[cfg(test)]
mod remove_element_test {
    use super::*;

    #[test]
    fn remove_element_test_1() {
        let mut arr = vec![3, 2, 2, 3];
        assert_eq!(remove_element(&mut arr, 3), 2);
        assert_eq!(arr, [2, 2]);
    }

    #[test]
    fn remove_element_test_2() {
        let mut arr = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(remove_element(&mut arr, 2), 5);
        assert_eq!(arr, [0, 1, 3, 0, 4]);
    }
}
