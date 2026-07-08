fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    fn build_board(n: usize, queen_xs: &Vec<usize>) -> Vec<Vec<bool>> {
        let mut board: Vec<Vec<bool>> = vec![vec![false; n]; n];
        let mut y = 0;
        for &x in queen_xs {
            for y_idx in 0..n {
                for x_idx in 0..n {
                    if y_idx == y || x_idx == x || x_idx.abs_diff(x) == y_idx.abs_diff(y) {
                        board[y_idx][x_idx] = true;
                    }
                }
            }
            y += 1;
        }
        board
    }

    fn backtrack(n: usize, queen_xs: &mut Vec<usize>, res: &mut Vec<Vec<usize>>) {
        if queen_xs.len() == n {
            res.push(queen_xs.clone());
            return;
        }

        let board = build_board(n, queen_xs);
        let row = queen_xs.len();
        for x in 0..n {
            if !board[row][x] {
                queen_xs.push(x);
                backtrack(n, queen_xs, res);
                queen_xs.pop();
            }
        }
    }

    let mut vec = vec![];
    backtrack(n, &mut vec![], &mut vec);
    let mut res = vec![];
    for v in vec {
        let mut board = vec![vec!['.'; n]; n];
        let mut y = 0;
        for x in v {
            board[y][x] = 'Q';
            y += 1;
        }
        res.push(
            board
                .iter()
                .map(|row| row.iter().collect::<String>())
                .collect(),
        );
    }
    res
}

#[cfg(test)]
mod solve_n_queens_test {
    use super::*;
    use crate::vector::normalise;

    #[test]
    fn solve_n_queens_test_1() {
        assert_eq!(
            normalise(solve_n_queens(4)),
            normalise(vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ])
        );
    }

    #[test]
    fn solve_n_queens_test_2() {
        assert_eq!(solve_n_queens(1), vec![vec!["Q"],]);
    }
}
