fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<char>>, path: (usize, usize), rows: usize, cols: usize) {
        if path.0 == rows || path.1 == cols || grid[path.0][path.1] == '0' {
            return;
        }
        grid[path.0][path.1] = '0';

        dfs(grid, (path.0 + 1, path.1), rows, cols);
        dfs(grid, (path.0, path.1 + 1), rows, cols);
        if path.0 != 0 {
            dfs(grid, (path.0 - 1, path.1), rows, cols);
        }
        if path.1 != 0 {
            dfs(grid, (path.0, path.1 - 1), rows, cols);
        }
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid = grid;
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                res += 1;
                dfs(&mut grid, (i, j), rows, cols);
            }
        }
    }
    res
}

// 1, 0, 1
// 1, 1, 1
// 1, 0, 1

#[cfg(test)]
mod number_of_islands_test {
    use super::*;

    #[test]
    fn number_of_islands_test_1() {
        assert_eq!(
            number_of_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
    }

    #[test]
    fn number_of_islands_test_2() {
        assert_eq!(
            number_of_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }

    #[test]
    fn number_of_islands_test_3() {
        assert_eq!(number_of_islands(vec![vec!['1']]), 1);
    }

    #[test]
    fn number_of_islands_test_4() {
        assert_eq!(
            number_of_islands(vec![
                vec!['1', '1', '1'],
                vec!['0', '1', '0'],
                vec!['1', '1', '1']
            ]),
            1
        );
    }
}
