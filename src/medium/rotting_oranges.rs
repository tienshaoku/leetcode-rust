fn rotting_oranges(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::{HashSet, VecDeque};

    let mut grid = grid;
    let width = grid[0].len();
    let height = grid.len();
    let mut queue = VecDeque::new();
    let mut set = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 2 {
                // swap 2 with -1 for later condition value comparison
                grid[y][x] = -1;
                queue.push_back((x, y, 0));
            } else if grid[y][x] == 1 {
                // swap 1 with i32::MAX for later condition value comparison
                grid[y][x] = i32::MAX;
                set.insert((x, y));
            }
        }
    }

    let mut res = 0;
    while let Some((x, y, l)) = queue.pop_front() {
        if x == width || y == height || (l > grid[y][x] && grid[y][x] != -1) {
            continue;
        }

        grid[y][x] = l;
        res = res.max(l);
        set.remove(&(x, y));

        let level = l + 1;
        queue.push_back((x + 1, y, level));
        queue.push_back((x, y + 1, level));
        if x > 0 {
            queue.push_back((x - 1, y, level));
        }
        if y > 0 {
            queue.push_back((x, y - 1, level));
        }
    }

    if set.is_empty() {
        res
    } else {
        -1
    }
}

#[cfg(test)]
mod rotting_oranges_test {
    use super::*;

    #[test]
    fn rotting_oranges_test_1() {
        assert_eq!(
            rotting_oranges(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1,]]),
            4
        );
    }

    #[test]
    fn rotting_oranges_test_2() {
        assert_eq!(
            rotting_oranges(vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1,]]),
            -1
        );
    }

    #[test]
    fn rotting_oranges_test_3() {
        assert_eq!(rotting_oranges(vec![vec![0, 2]]), 0);
    }

    #[test]
    fn rotting_oranges_test_4() {
        assert_eq!(rotting_oranges(vec![vec![1, 2, 1, 1, 1]]), 3);
    }
}
