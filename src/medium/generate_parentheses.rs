fn generate_parenthesis(n: i32) -> Vec<String> {
    fn traverse(n: i32, open: i32, close: i32, path: &mut String, res: &mut Vec<String>) {
        if path.len() == n as usize * 2 {
            res.push(path.clone());
            return;
        }

        if n > open {
            path.push('(');
            traverse(n, open + 1, close, path, res);
            path.pop();
        }
        // also independent condition s.t. when both are valid, try ')' after
        // the '(' subtree is fully explored. Also runs when no more '(' is allowed
        if open > close {
            path.push(')');
            traverse(n, open, close + 1, path, res);
            path.pop();
        }
    }

    let mut res = vec![];
    traverse(n, 0, 0, &mut String::new(), &mut res);
    res
}

// instead of "(" + G(i) + G(k - i - 1) + ")" which limits the start and end,
// the pattern should be G(k) = "(" + G(i) + ")" + G(k - i - 1) to cover all cases
fn generate_parenthesis_dp(n: i32) -> Vec<String> {
    let mut arr = vec![vec![String::new()]];

    for i in 1..n as usize + 1 {
        let mut res = vec![];
        for j in 0..i {
            for k in &arr[j] {
                for l in &arr[i - j - 1] {
                    res.push(format!("({}){}", k, l));
                }
            }
        }
        arr.push(res);
    }
    arr.last().unwrap().to_vec()
}

#[cfg(test)]
mod generate_parenthesis_test {
    use super::*;
    use crate::vector::normalise;

    #[test]
    fn generate_parenthesis_test_1() {
        assert_eq!(normalise(generate_parenthesis(1)), normalise(vec!["()"]));
    }

    #[test]
    fn generate_parenthesis_test_2() {
        assert_eq!(
            normalise(generate_parenthesis(2)),
            normalise(vec!["(())", "()()"])
        );
    }

    #[test]
    fn generate_parenthesis_test_3() {
        assert_eq!(
            normalise(generate_parenthesis(3)),
            normalise(vec!["((()))", "(()())", "(())()", "()(())", "()()()"])
        );
    }

    #[test]
    fn generate_parenthesis_test_4() {
        assert_eq!(
            normalise(generate_parenthesis(4)),
            normalise(vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ])
        );
    }
}
