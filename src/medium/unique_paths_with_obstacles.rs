fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let m = obstacle_grid.len();
    let n = obstacle_grid[0].len();

    let mut arr = vec![vec![0; n]; m];
    if obstacle_grid[m - 1][n - 1] == 1 {
        return 0;
    } else {
        arr[m - 1][n - 1] = 1;
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m - 1 && j == n - 1 {
                continue;
            }
            if obstacle_grid[i][j] == 1 {
                arr[i][j] = 0;
                continue;
            }

            let right = if j < n - 1 { arr[i][j + 1] } else { 0 };
            let down = if i < m - 1 { arr[i + 1][j] } else { 0 };
            arr[i][j] = right + down;
        }
    }
    arr[0][0]
}

#[cfg(test)]
mod unique_paths_with_obstacles_test {
    use super::*;

    #[test]
    fn unique_paths_with_obstacles_test_1() {
        assert_eq!(
            unique_paths_with_obstacles(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            2
        );
    }

    #[test]
    fn unique_paths_with_obstacles_test_2() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]), 1);
    }

    #[test]
    fn unique_paths_with_obstacles_test_3() {
        assert_eq!(
            unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 0, 0]
            ]),
            3
        );
    }

    #[test]
    fn unique_paths_with_obstacles_test_4() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]]), 0);
    }

    #[test]
    fn unique_paths_with_obstacles_test_5() {
        assert_eq!(unique_paths_with_obstacles(vec![vec![0, 1, 0, 0]]), 0);
    }

    #[test]
    fn unique_paths_with_obstacles_test_6() {
        assert_eq!(
            unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            0
        );
    }
}
