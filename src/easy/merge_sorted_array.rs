fn merge_sorted_array(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    use std::collections::VecDeque;

    if m == 0 {
        *nums1 = nums2.clone();
        return;
    }
    if n == 0 {
        return;
    }

    let mut index = 0;
    let mut cloned = VecDeque::from(nums1[0..m as usize].to_vec());
    for i in 0..m + n {
        let value = match (cloned.is_empty(), index == n as usize) {
            (true, _) => {
                let val = nums2[index];
                index += 1;
                val
            }
            (false, true) => cloned.pop_front().unwrap(),
            (false, false) => {
                let front = *cloned.front().unwrap();
                if front <= nums2[index] {
                    cloned.pop_front();
                    front
                } else {
                    let val = nums2[index];
                    index += 1;
                    val
                }
            }
        };

        nums1[i as usize] = value;
    }
}

#[cfg(test)]
mod merge_sorted_array_test {
    use super::*;

    #[test]
    fn merge_sorted_array_test_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        merge_sorted_array(&mut nums1, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(nums1, [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn merge_sorted_array_test_2() {
        let mut nums1 = vec![1];
        merge_sorted_array(&mut nums1, 1, &mut vec![], 0);
        assert_eq!(nums1, [1]);
    }

    #[test]
    fn merge_sorted_array_test_3() {
        let mut nums1 = vec![0];
        merge_sorted_array(&mut nums1, 0, &mut vec![1], 1);
        assert_eq!(nums1, [1]);
    }

    #[test]
    fn merge_sorted_array_test_4() {
        let mut nums1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        merge_sorted_array(&mut nums1, 6, &mut vec![1, 2, 2], 3);
        assert_eq!(nums1, [-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}
