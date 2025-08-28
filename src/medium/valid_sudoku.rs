fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    // don't even need to know which is horizontal, which vertical
    // just use diff index and both directions will be checked anyways
    let mut one = vec![HashSet::<u32>::new(); 9];
    let mut two = vec![HashSet::<u32>::new(); 9];
    for b in 0..9 {
        let mut set = HashSet::new();
        for v in 0..3 {
            for h in 0..3 {
                if board[v + b / 3 * 3][h + b % 3 * 3] != '.' {
                    let value = board[v + b / 3 * 3][h + b % 3 * 3] as u32;

                    // 3*3 box
                    if !set.insert(value) {
                        return false;
                    }

                    if !one[v + b / 3 * 3].insert(value) {
                        return false;
                    }
                    if !two[h + b % 3 * 3].insert(value) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn is_valid_sudoku_redundant(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    for v in 0..9 {
        let mut set = HashSet::new();
        for h in 0..9 {
            if board[v][h] != '.' {
                let value = board[v][h] as u32;
                if !set.insert(value) {
                    return false;
                }
            }
        }
    }

    for v in 0..9 {
        let mut set = HashSet::new();
        for h in 0..9 {
            if board[h][v] != '.' {
                let value = board[h][v] as u32;
                if !set.insert(value) {
                    return false;
                }
            }
        }
    }

    for b in 0..9 {
        let mut set = HashSet::new();
        for v in 0..3 {
            for h in 0..3 {
                if board[v + b / 3 * 3][h + b % 3 * 3] != '.' {
                    let value = board[v + b / 3 * 3][h + b % 3 * 3] as u32;
                    if !set.insert(value) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod is_valid_sudoku_test {
    use super::*;

    #[test]
    fn is_valid_sudoku_test_1() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }

    #[test]
    fn is_valid_sudoku_test_2() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }

    #[test]
    fn is_valid_sudoku_test_3() {
        assert_eq!(
            is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '2'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }
}
