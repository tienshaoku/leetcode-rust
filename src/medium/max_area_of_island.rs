fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<i32>>, path: (usize, usize), rows: usize, cols: usize) -> i32 {
        if path.0 == rows || path.1 == cols || grid[path.0][path.1] == 0 {
            return 0;
        }
        grid[path.0][path.1] = 0;

        let mut area = 1;

        area += dfs(grid, (path.0 + 1, path.1), rows, cols);
        area += dfs(grid, (path.0, path.1 + 1), rows, cols);
        if path.0 != 0 {
            area += dfs(grid, (path.0 - 1, path.1), rows, cols);
        }
        if path.1 != 0 {
            area += dfs(grid, (path.0, path.1 - 1), rows, cols);
        }
        area
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid = grid;
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                res = res.max(dfs(&mut grid, (i, j), rows, cols));
            }
        }
    }
    res
}

#[cfg(test)]
mod max_area_of_island_test {
    use super::*;

    #[test]
    fn max_area_of_island_test_1() {
        assert_eq!(
            max_area_of_island(vec![
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
                vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            6
        );
    }

    #[test]
    fn max_area_of_island_test_2() {
        assert_eq!(max_area_of_island(vec![vec![0, 0, 0, 0, 0, 0, 0, 0]]), 0);
    }
}
