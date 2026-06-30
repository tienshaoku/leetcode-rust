use crate::vector::normalise;

fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn traverse(
        candidates: &Vec<i32>,
        target: i32,
        start_index: usize,
        res: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if target == 0 {
            res.push(path.clone());
            return;
        }

        for i in start_index..candidates.len() {
            let now = candidates[i];
            if now > target {
                break;
            }

            path.push(now);
            traverse(candidates, target - now, i, res, path);
            // backtracking
            path.pop();
        }
    }

    let mut res = vec![];
    let mut cloned = candidates.clone();
    cloned.sort();
    traverse(&cloned, target, 0, &mut res, &mut vec![]);
    res
}

fn combination_sum_duplicates(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    fn traverse(candidates: &Vec<i32>, target: i32, arr: &Vec<i32>, res: &mut HashSet<Vec<i32>>) {
        if candidates.is_empty() {
            if target == 0 {
                res.insert(arr.to_vec());
            }
            return;
        }

        for i in candidates {
            if *i > target {
                continue;
            }
            let rest = target - i;
            let mut arr_clone = arr.clone();
            arr_clone.push(*i);
            arr_clone.sort();
            let rest_candidates: Vec<i32> =
                candidates.iter().filter(|x| **x <= rest).copied().collect();
            traverse(&rest_candidates, rest, &arr_clone, res);
        }
    }

    let mut res = HashSet::new();
    traverse(&candidates, target, &vec![], &mut res);
    res.into_iter().collect()
}

#[cfg(test)]
mod combination_sum_test {
    use super::*;

    #[test]
    fn combination_sum_test_1() {
        assert_eq!(
            normalise(combination_sum(vec![2, 3, 6, 7], 7)),
            normalise(vec![vec![2, 2, 3], vec![7]])
        );
    }

    #[test]
    fn combination_sum_test_2() {
        assert_eq!(
            normalise(combination_sum(vec![2, 3, 5], 8)),
            normalise(vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]])
        );
    }

    #[test]
    fn combination_sum_test_3() {
        assert_eq!(combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn combination_sum_test_4() {
        assert_eq!(
            normalise(combination_sum(vec![8, 7, 4, 3], 11)),
            normalise(vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]])
        );
    }
}
