fn number_of_islands(grid: Vec<Vec<char>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
        if x == grid[0].len() || y == grid.len() || grid[y][x] == '0' {
            return;
        }
        grid[y][x] = '0';

        dfs(grid, x + 1, y);
        dfs(grid, x, y + 1);
        if x > 0 {
            dfs(grid, x - 1, y);
        }
        if y > 0 {
            dfs(grid, x, y - 1);
        }
    }

    let mut count = 0;
    let mut grid = grid;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '1' {
                count += 1;
                dfs(&mut grid, x, y);
            }
        }
    }
    count
}

// 1, 0, 1
// 1, 1, 1
// 1, 0, 1

// 1, 1, 1
// 0, 1, 0
// 1, 1, 1

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
