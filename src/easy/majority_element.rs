fn majority_element(nums: Vec<i32>) -> i32 {
    let mut current = nums[0];
    let mut count = 1;
    for i in 1..nums.len() {
        if nums[i] == current {
            count += 1;
        } else {
            if count > 1 {
                count -= 1;
            } else {
                current = nums[i];
                count = 1;
            }
        }
    }
    current
}

#[cfg(test)]
mod majority_element_test {
    use super::*;

    #[test]
    fn test_majority_element_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test_majority_element_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
