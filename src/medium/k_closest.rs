fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut points = points;
    points.sort_by_key(|v| v[0] * v[0] + v[1] * v[1]);
    points.truncate(k as usize);
    points
}

fn k_closest_slow(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    use std::collections::BinaryHeap;

    let mut heap: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();
    for i in points {
        let x = i[0];
        let y = i[1];
        let dis = x * x + y * y;

        if heap.len() < k as usize {
            heap.push((dis, x, y));
        } else if dis < heap.peek().unwrap().0 {
            heap.pop();
            heap.push((dis, x, y));
        }
    }
    heap.into_iter().map(|(_, x, y)| vec![x, y]).collect()
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
            vec![vec![3, 3], vec![-2, 4]]
        );
    }
}
