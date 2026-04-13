fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let height = matrix.len();
    let width = matrix[0].len();
    let (mut t, mut b) = (0, height);

    let (mut h, mut w);
    while t < b {
        h = t + (b - t) / 2;
        if matrix[h][0] > target {
            b = h;
            continue;
        } else if matrix[h][width - 1] < target {
            t = h + 1;
            continue;
        }

        let (mut l, mut r) = (0, width);
        while l < r {
            w = l + (r - l) / 2;
            if matrix[h][w] == target {
                return true;
            } else if matrix[h][w] > target {
                r = w;
            } else {
                l = w + 1;
            }
        }
        return false;
    }
    false
}

fn search_matrix_longer(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    use std::cmp::Ordering;

    let (mut top, mut bottom) = (0, matrix.len());
    let mut row = 0;
    while top < bottom {
        let mid = top + (bottom - top) / 2;
        let last = *matrix[mid].last().unwrap();
        if target == matrix[mid][0] || target == last {
            return true;
        }
        match (target > matrix[mid][0], target > last) {
            (true, true) => top = mid + 1,
            (true, false) => {
                row = mid;
                break;
            }
            (false, false) => bottom = mid,
            _ => (),
        }
    }
    let (mut left, mut right) = (0, matrix[row].len());
    while left < right {
        let mid = left + (right - left) / 2;
        match target.cmp(&matrix[row][mid]) {
            Ordering::Equal => return true,
            Ordering::Greater => left = mid + 1,
            _ => right = mid,
        }
    }
    false
}

#[cfg(test)]
mod search_matrix_test {
    use super::*;

    #[test]
    fn search_matrix_test_1() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
    }

    #[test]
    fn search_matrix_test_2() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }

    #[test]
    fn search_matrix_test_3() {
        assert_eq!(search_matrix(vec![vec![1]], 1), true);
    }

    #[test]
    fn search_matrix_test_4() {
        assert_eq!(search_matrix(vec![vec![1]], 15), false);
    }
}
