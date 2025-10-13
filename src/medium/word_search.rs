fn word_search(board: Vec<Vec<char>>, word: String) -> bool {
    fn search(
        board: &Vec<Vec<char>>,
        chars: &Vec<char>,
        index: usize,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if index < chars.len() && board[y][x] == chars[index] && !visited[y][x] {
            if index == chars.len() - 1 {
                return true;
            }

            visited[y][x] = true;
            let result = (x > 0 && search(board, chars, index + 1, x - 1, y, visited))
                || (x + 1 < board[0].len() && search(board, chars, index + 1, x + 1, y, visited))
                || (y > 0 && search(board, chars, index + 1, x, y - 1, visited))
                || (y + 1 < board.len() && search(board, chars, index + 1, x, y + 1, visited));

            if !result {
                visited[y][x] = false;
                return false;
            }
            true
        } else {
            false
        }
    }

    let chars: Vec<char> = word.chars().collect();
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            // only start if the first one matches
            if board[y][x] == chars[0] {
                if search(&board, &chars, 0, x, y, &mut visited) {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod word_search_test {
    use super::*;

    #[test]
    fn word_search_test_1() {
        assert_eq!(
            word_search(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCCED")
            ),
            true
        );
    }

    #[test]
    fn word_search_test_2() {
        assert_eq!(
            word_search(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("SEE")
            ),
            true
        );
    }

    #[test]
    fn word_search_test_3() {
        assert_eq!(
            word_search(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCB")
            ),
            false
        );
    }
}
