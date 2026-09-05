fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut new_interval = new_interval;
    let mut i = 0;
    let n = intervals.len();

    while i < n && intervals[i][1] < new_interval[0] {
        res.push(intervals[i].clone());
        i += 1;
    }
    while i < n && intervals[i][0] <= new_interval[1] {
        new_interval[0] = new_interval[0].min(intervals[i][0]);
        new_interval[1] = new_interval[1].max(intervals[i][1]);
        i += 1;
    }
    res.push(new_interval);
    while i < n {
        res.push(intervals[i].clone());
        i += 1;
    }
    res
}

#[cfg(test)]
mod insert_interval_test {
    use super::*;

    #[test]
    fn insert_interval_test_1() {
        assert_eq!(
            insert_interval(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            [[1, 5], [6, 9]]
        );
    }

    #[test]
    fn insert_interval_test_2() {
        assert_eq!(
            insert_interval(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            [[1, 2], [3, 10], [12, 16]]
        );
    }

    #[test]
    fn insert_interval_test_3() {
        assert_eq!(insert_interval(vec![], vec![5, 7]), [[5, 7]]);
    }

    #[test]
    fn insert_interval_test_4() {
        assert_eq!(insert_interval(vec![vec![1, 5]], vec![2, 7]), [[1, 7]]);
    }
}
