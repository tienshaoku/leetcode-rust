use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct MedianFinder {
    min: BinaryHeap<Reverse<i32>>,
    max: BinaryHeap<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            min: BinaryHeap::new(),
            max: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.max.is_empty() && self.min.is_empty() {
            self.max.push(num);
            return;
        }

        match self.max.len().cmp(&self.min.len()) {
            Ordering::Equal => {
                if num <= self.min.peek().unwrap().0 {
                    self.max.push(num);
                } else {
                    self.min.push(Reverse(num));
                }
            }
            Ordering::Greater => {
                if num >= *self.max.peek().unwrap() {
                    self.min.push(Reverse(num));
                } else {
                    let popped = self.max.pop().unwrap();
                    self.max.push(num);
                    self.min.push(Reverse(popped));
                }
            }
            _ => {
                if num <= self.min.peek().unwrap().0 {
                    self.max.push(num);
                } else {
                    let popped = self.min.pop().unwrap().0;
                    self.min.push(Reverse(num));
                    self.max.push(popped);
                }
            }
        }
    }

    fn find_median(&self) -> f64 {
        match self.max.len().cmp(&self.min.len()) {
            Ordering::Equal => {
                (self.min.peek().unwrap().0 as f64 + *self.max.peek().unwrap() as f64) / 2.0
            }
            Ordering::Greater => *self.max.peek().unwrap() as f64,
            _ => self.min.peek().unwrap().0 as f64,
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

#[cfg(test)]
mod find_median_test {
    use super::*;

    #[test]
    fn find_median_test_1() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert_eq!(median_finder.find_median(), 1.5);
        median_finder.add_num(3);
        assert_eq!(median_finder.find_median(), 2.0);
    }
}
