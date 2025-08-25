// instead of "(" + G(i) + G(k - i - 1) + ")" which limits the start and end,
// the pattern should be G(k) = "(" + G(i) + ")" + G(k - i - 1) to cover all cases
fn generate_parenthesis(n: i32) -> Vec<String> {
    use std::collections::HashSet;
    let mut res: Vec<Vec<String>> = vec![vec!["".to_string()]];

    for k in 1..=n as usize {
        let mut set = HashSet::new();
        for i in 0..k {
            for a in &res[i] {
                for b in &res[k - i - 1] {
                    set.insert(format!("({}){}", a, b));
                }
            }
        }
        // let mut sorted: Vec<String> = set.into_iter().collect();
        // sorted.sort();
        // res.push(sorted);

        res.push(set.into_iter().collect());
    }

    res[n as usize].clone()
}

#[cfg(test)]
mod generate_parenthesis_test {
    use super::*;

    #[test]
    fn generate_parenthesis_test_1() {
        assert_eq!(generate_parenthesis(1), vec!["()"]);
    }

    #[test]
    fn generate_parenthesis_test_2() {
        assert_eq!(generate_parenthesis(2), vec!["(())", "()()"]);
    }

    #[test]
    fn generate_parenthesis_test_3() {
        assert_eq!(
            generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }

    #[test]
    fn generate_parenthesis_test_4() {
        assert_eq!(
            generate_parenthesis(4),
            vec![
                "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
                "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
            ]
        );
    }
}
