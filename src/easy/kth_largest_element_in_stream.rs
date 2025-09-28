use std::collections::BinaryHeap;
struct KthLargest {
    heap: BinaryHeap<i32>,
    k: i32,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let range = if nums.len() < k as usize {
            0..nums.len()
        } else {
            (nums.len() - k as usize)..nums.len()
        };

        let mut arr = nums.clone();
        arr.sort();

        Self {
            heap: arr[range].into_iter().map(|&x| -x).collect(),
            k,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k as usize {
            self.heap.push(-val);
        } else if -val < *self.heap.peek().unwrap() {
            self.heap.pop();
            self.heap.push(-val);
        }
        -self.heap.peek().unwrap()
    }
}

// fatter but easier
// impl KthLargest {
//     fn new(k: i32, nums: Vec<i32>) -> Self {
//         Self {
//             k,
//             heap: BinaryHeap::from_iter(nums.iter().map(|n| -n)),
//         }
//     }

//     fn add(&mut self, val: i32) -> i32 {
//         self.heap.push(-val);
//         while self.heap.len() > self.k as usize {
//             self.heap.pop();
//         }
//         -*self.heap.peek().unwrap()
//     }
// }

#[cfg(test)]
mod kth_largest_element_in_stream_test {
    use super::*;

    #[test]
    fn kth_largest_element_in_stream_test_1() {
        let mut kth = KthLargest::new(3, vec![4, 5, 8, 2]);
        assert_eq!(kth.add(3), 4);
        assert_eq!(kth.add(5), 5);
        assert_eq!(kth.add(10), 5);
        assert_eq!(kth.add(9), 8);
        assert_eq!(kth.add(4), 8);
    }

    #[test]
    fn kth_largest_element_in_stream_test_2() {
        let mut kth = KthLargest::new(4, vec![7, 7, 7, 7, 8, 3]);
        assert_eq!(kth.add(2), 7);
        assert_eq!(kth.add(10), 7);
        assert_eq!(kth.add(9), 7);
        assert_eq!(kth.add(9), 8);
    }

    #[test]
    fn kth_largest_element_in_stream_test_3() {
        let mut kth = KthLargest::new(1, vec![]);
        assert_eq!(kth.add(-3), -3);
        assert_eq!(kth.add(-2), -2);
        assert_eq!(kth.add(-4), -2);
        assert_eq!(kth.add(0), 0);
        assert_eq!(kth.add(4), 4);
    }

    #[test]
    fn kth_largest_element_in_stream_test_4() {
        let mut kth = KthLargest::new(2, vec![0]);
        assert_eq!(kth.add(-1), -1);
        assert_eq!(kth.add(1), 0);
        assert_eq!(kth.add(-2), 0);
        assert_eq!(kth.add(-4), 0);
        assert_eq!(kth.add(3), 1);
    }

    #[test]
    fn kth_largest_element_in_stream_test_5() {
        let mut kth = KthLargest::new(3, vec![5, -1]);
        assert_eq!(kth.add(2), -1);
        assert_eq!(kth.add(1), 1);
        assert_eq!(kth.add(-1), 1);
        assert_eq!(kth.add(3), 2);
        assert_eq!(kth.add(4), 3);
    }
}
