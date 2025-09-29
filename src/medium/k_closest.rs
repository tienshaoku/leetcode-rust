fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for i in points {
        let new_distance = i[0].pow(2) + i[1].pow(2);
        if heap.len() < k as usize {
            heap.push((new_distance, i[0], i[1]));
        } else {
            let top = heap.peek().unwrap();
            if new_distance < top.1.pow(2) + top.2.pow(2) {
                heap.pop();
                heap.push((new_distance, i[0], i[1]));
            }
        }
    }
    heap.iter().map(|&(_, b, c)| vec![b, c]).collect()
}

#[cfg(test)]
mod k_closest_test {
    use super::*;

    #[test]
    fn k_closest_test_1() {
        assert_eq!(
            k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
    }

    #[test]
    fn k_closest_test_2() {
        assert_eq!(
            k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![-2, 4], vec![3, 3]]
        );
    }
}
