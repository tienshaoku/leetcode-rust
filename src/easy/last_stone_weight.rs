use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);
    while heap.len() > 1 {
        let first = heap.pop().unwrap();
        let second = heap.pop().unwrap();
        heap.push(first - second);
    }
    match heap.len() {
        // use peek instead of pop the last time is slightly faster
        1 => *heap.peek().unwrap(),
        _ => 0,
    }
}

fn last_stone_weight_slower(stones: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::from(stones);
    loop {
        match (heap.pop(), heap.pop()) {
            (Some(a), Some(b)) => {
                if a != b {
                    // cuz it's a BinaryHeap
                    heap.push(a - b)
                };
            }
            (Some(v), None) => return v,
            (None, _) => return 0,
        }
    }
}

#[cfg(test)]
mod last_stone_weight_test {
    use super::*;

    #[test]
    fn last_stone_weight_test_1() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn last_stone_weight_test_2() {
        assert_eq!(last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn last_stone_weight_test_3() {
        assert_eq!(last_stone_weight(vec![2, 2]), 0);
    }
}
