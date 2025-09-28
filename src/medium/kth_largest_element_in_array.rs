fn kth_largest_element_in_array(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new();
    heap.push(-nums[0]);
    for i in 1..nums.len() {
        match (heap.len() < k as usize, -nums[i] < *heap.peek().unwrap()) {
            (true, _) => {
                heap.push(-nums[i]);
            }
            (false, true) => {
                heap.pop();
                heap.push(-nums[i]);
            }
            _ => (),
        }
    }
    -*heap.peek().unwrap()
}

#[cfg(test)]
mod kth_largest_element_in_array_test {
    use super::*;

    #[test]
    fn kth_largest_element_in_array_test_1() {
        assert_eq!(kth_largest_element_in_array(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn kth_largest_element_in_array_test_2() {
        assert_eq!(
            kth_largest_element_in_array(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn kth_largest_element_in_array_test_3() {
        assert_eq!(kth_largest_element_in_array(vec![1], 1), 1);
    }
}
