fn course_schedule_two(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::{HashMap, VecDeque};
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut pre_count = vec![0; num_courses as usize];
    for p in prerequisites {
        if p.len() != 2 {
            continue;
        }
        map.entry(p[1]).or_insert(vec![]).push(p[0]);
        pre_count[p[0] as usize] += 1;
    }

    let mut queue = VecDeque::new();
    for i in 0..pre_count.len() {
        if pre_count[i] == 0 {
            queue.push_back(i as i32);
        }
    }

    let mut path = vec![];
    while let Some(i) = queue.pop_front() {
        path.push(i);

        if let Some(vec) = map.get(&i) {
            for &v in vec {
                pre_count[v as usize] -= 1;
                if pre_count[v as usize] == 0 {
                    queue.push_back(v);
                }
            }
        }
    }

    if path.len() as i32 == num_courses {
        path
    } else {
        vec![]
    }
}

#[cfg(test)]
mod course_schedule_two_test {
    use super::*;

    #[test]
    fn course_schedule_two_test_1() {
        assert_eq!(course_schedule_two(2, vec![vec![1, 0]]), [0, 1]);
    }

    #[test]
    fn course_schedule_two_test_2() {
        assert_eq!(
            course_schedule_two(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
            [0, 1, 2, 3]
        );
    }

    #[test]
    fn course_schedule_two_test_3() {
        assert_eq!(course_schedule_two(1, vec![vec![]]), [0]);
    }

    #[test]
    fn course_schedule_two_test_4() {
        assert_eq!(course_schedule_two(2, vec![vec![1, 0], vec![0, 1]]), []);
    }
}
