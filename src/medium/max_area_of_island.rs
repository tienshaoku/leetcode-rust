fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, area: &mut i32) {
        if x == grid[0].len() || y == grid.len() || grid[y][x] == 0 {
            return;
        }
        grid[y][x] = 0;
        *area += 1;

        dfs(grid, x + 1, y, area);
        dfs(grid, x, y + 1, area);
        if x > 0 {
            dfs(grid, x - 1, y, area);
        }
        if y > 0 {
            dfs(grid, x, y - 1, area);
        }
    }

    let mut area = 0;
    let mut grid = grid;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 1 {
                let mut tmp = 0;
                dfs(&mut grid, x, y, &mut tmp);
                area = area.max(tmp);
            }
        }
    }
    area
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
