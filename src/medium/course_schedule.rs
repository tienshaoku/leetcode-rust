fn course_schedule(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for p in prerequisites {
        map.entry(p[0]).or_insert(vec![]).push(p[1]);
    }

    fn dfs(target: i32, map: &HashMap<i32, Vec<i32>>, state: &mut Vec<u8>) -> bool {
        match state[target as usize] {
            1 => return false,
            2 => return true,
            _ => (),
        }
        state[target as usize] = 1;
        if let Some(vec) = map.get(&target) {
            for &v in vec {
                if !dfs(v, map, state) {
                    return false;
                }
            }
        }
        state[target as usize] = 2;
        true
    }

    let mut state = vec![0u8; num_courses as usize];
    for i in 0..num_courses {
        if !dfs(i, &map, &mut state) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod course_schedule_test {
    use super::*;

    #[test]
    fn course_schedule_test_1() {
        assert_eq!(course_schedule(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn course_schedule_test_2() {
        assert_eq!(course_schedule(2, vec![vec![1, 0], vec![0, 1]]), false);
    }

    #[test]
    fn course_schedule_test_3() {
        assert_eq!(
            course_schedule(3, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            true
        );
    }

    #[test]
    fn course_schedule_test_4() {
        assert_eq!(
            course_schedule(3, vec![vec![0, 2], vec![1, 2], vec![2, 0]]),
            false
        );
    }

    #[test]
    fn course_schedule_test_5() {
        assert_eq!(
            course_schedule(
                20,
                vec![
                    vec![0, 10],
                    vec![3, 18],
                    vec![5, 5],
                    vec![6, 11],
                    vec![11, 14],
                    vec![13, 1],
                    vec![15, 1],
                    vec![17, 4]
                ]
            ),
            false
        );
    }
}
