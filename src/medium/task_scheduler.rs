fn task_scheduler(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::HashMap;

    if n == 0 {
        return tasks.len() as i32;
    }

    let mut map = HashMap::new();
    for i in &tasks {
        *map.entry(i).or_insert(0) += 1;
    }

    let max_count = *map.values().max().unwrap();
    let num_of_max = map.values().filter(|&&x| x == max_count).count();

    // non_last_bucket: (max_count - 1) * (n + 1); max_count - 1 to count all buckets but the last
    // last_bucket: put all max_counts into the last bucket
    (tasks.len() as i32).max((max_count - 1) * (n + 1) + num_of_max as i32)
}

fn task_scheduler_slow(tasks: Vec<char>, n: i32) -> i32 {
    use std::cmp::Reverse;
    use std::collections::HashMap;
    if n == 0 {
        return tasks.len() as i32;
    }

    let mut map = HashMap::new();
    for i in &tasks {
        *map.entry(i).or_insert(0) += 1;
    }
    let mut arr: Vec<(i32, char)> = map.into_iter().map(|(k, v)| (v, *k)).collect();

    let mut total = tasks.len();
    let mut res = 0;
    let mut count_now = 0;
    let mut tmp_arr = vec![];
    while total > 0 {
        arr.sort_by_key(|x| Reverse(x.0));
        if count_now == 0 {
            tmp_arr.clear();
            tmp_arr.push(arr[0].1);
            arr[0].0 -= 1;
            total -= 1;
        } else {
            for i in arr.iter_mut() {
                if !tmp_arr.contains(&i.1) && i.0 != 0 {
                    tmp_arr.push(i.1);
                    i.0 -= 1;
                    total -= 1;
                    break;
                }
            }
        }

        count_now = (count_now + 1) % (n + 1);
        res += 1;
    }
    res
}

#[cfg(test)]
mod task_scheduler_test {
    use super::*;

    #[test]
    fn task_scheduler_test_1() {
        assert_eq!(task_scheduler(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
    }

    #[test]
    fn task_scheduler_test_2() {
        assert_eq!(task_scheduler(vec!['A', 'C', 'A', 'B', 'D', 'B'], 1), 6);
    }

    #[test]
    fn task_scheduler_test_3() {
        assert_eq!(task_scheduler(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3), 10);
    }

    #[test]
    fn task_scheduler_test_4() {
        assert_eq!(task_scheduler(vec!['A'], 0), 1);
    }

    #[test]
    fn task_scheduler_test_5() {
        assert_eq!(task_scheduler(vec!['A'], 3), 1);
    }

    #[test]
    fn task_scheduler_test_6() {
        assert_eq!(task_scheduler(vec!['A', 'A'], 2), 4);
    }

    #[test]
    fn task_scheduler_test_7() {
        assert_eq!(
            task_scheduler(
                vec!['A', 'A', 'A', 'B', 'B', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K'],
                7
            ),
            18
        );
    }
}
