fn task_scheduler(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::HashMap;

    let (max, num_of_max) = tasks
        .iter()
        .fold(HashMap::new(), |mut map, &i| {
            *map.entry(i).or_insert(0) += 1;
            map
        })
        .into_iter()
        .fold((0, 0), |(mut max, mut num_of_max), (_, v)| {
            if v > max {
                max = v;
                num_of_max = 1;
            } else if v == max {
                num_of_max += 1;
            }
            (max, num_of_max)
        });

    // there needs to have (max - 1) non-last bucket. Each non-last bucket is of length (n + 1)
    // last bucket: put all num_of_max into the last bucket
    // if sum(non_last_bucket + last_bucket) > tasks.len(), there is idle period
    (tasks.len() as i32).max((max - 1) * (n + 1) + num_of_max)
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
