fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut arr = vec![vec![0; n]; m];
    arr[m - 1][n - 1] = grid[m - 1][n - 1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m - 1 && j == n - 1 {
                continue;
            }
            if j == n - 1 {
                arr[i][j] = grid[i][j] + arr[i + 1][j];
            } else if i == m - 1 {
                arr[i][j] = grid[i][j] + arr[i][j + 1];
            } else {
                arr[i][j] = grid[i][j] + arr[i][j + 1].min(arr[i + 1][j]);
            }
        }
    }
    arr[0][0]
}

#[cfg(test)]
mod min_path_sum_test {
    use super::*;

    #[test]
    fn min_path_sum_test_1() {
        assert_eq!(
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }

    #[test]
    fn min_path_sum_test_2() {
        assert_eq!(min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
