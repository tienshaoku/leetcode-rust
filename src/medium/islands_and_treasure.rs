// use dfs is bumps time limit
fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let width = grid[0].len();
    let height = grid.len();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                queue.push_back((x, y, 0));
            }
        }
    }

    while let Some((x, y, l)) = queue.pop_front() {
        // explore by level, and thus each node will be explored only once
        // s.t. skipping condition can be v strict and efficient: grid[y][x] != i32::MAX
        if x == width || y == height || (grid[y][x] != i32::MAX && l != 0) {
            continue;
        }

        grid[y][x] = l;
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
}

#[cfg(test)]
mod islands_and_treasure_test {
    use super::*;

    #[test]
    fn islands_and_treasure_test_1() {
        let mut arr = vec![
            vec![i32::MAX, -1, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, i32::MAX, -1],
            vec![i32::MAX, -1, i32::MAX, -1],
            vec![0, -1, i32::MAX, i32::MAX],
        ];
        islands_and_treasure(&mut arr);
        assert_eq!(
            arr,
            [[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 4]]
        );
    }

    #[test]
    fn islands_and_treasure_test_2() {
        let mut arr = vec![vec![0, -1], vec![i32::MAX, i32::MAX]];
        islands_and_treasure(&mut arr);
        assert_eq!(arr, [[0, -1], [1, 2]]);
    }
}
