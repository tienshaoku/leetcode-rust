use crate::vector::normalise;

fn combination_sum_two(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
        if start_index >= candidates.len() {
            return;
        }

        for i in start_index..candidates.len() {
            let now = candidates[i];
            if now > target {
                break;
            }

            // when == start_index: first iteration
            // thus, if i > start_index (not the first iteration),
            // and the value is the same as the previous, can skip
            if i > start_index && now == candidates[i - 1] {
                continue;
            }

            path.push(now);
            traverse(candidates, target - now, i + 1, res, path);
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

#[cfg(test)]
mod combination_sum_two_test {
    use super::*;

    #[test]
    fn combination_sum_two_test_1() {
        assert_eq!(
            normalise(combination_sum_two(vec![10, 1, 2, 7, 6, 1, 5], 8)),
            normalise(vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]])
        );
    }

    #[test]
    fn combination_sum_two_test_2() {
        assert_eq!(
            normalise(combination_sum_two(vec![2, 5, 2, 1, 2], 5)),
            normalise(vec![vec![1, 2, 2], vec![5]])
        );
    }
}
