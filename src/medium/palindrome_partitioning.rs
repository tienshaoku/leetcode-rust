fn palindrome_partitioning(s: String) -> Vec<Vec<String>> {
    fn is_palindrome(chars: &Vec<char>, l: usize, r: usize) -> bool {
        let mut i = l;
        let mut j = r;
        while i < j {
            if chars[i] != chars[j] {
                return false;
            }
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        true
    }

    fn backtracking(
        start: usize,
        chars: &Vec<char>,
        length: usize,
        path: &mut Vec<String>,
        res: &mut Vec<Vec<String>>,
    ) {
        if start == length {
            res.push(path.clone());
            return;
        }
        for end in start..length {
            if is_palindrome(chars, start, end) {
                let substring = chars[start..=end].iter().collect::<String>();
                path.push(substring);
                backtracking(end + 1, chars, length, path, res);
                path.pop();
            }
        }
    }

    let chars = s.chars().collect::<Vec<char>>();
    let mut res = vec![];
    backtracking(0, &chars, chars.len(), &mut vec![], &mut res);
    res
}

#[cfg(test)]
mod palindrome_partitioning_test {
    use super::*;

    #[test]
    fn palindrome_partitioning_test_1() {
        assert_eq!(
            palindrome_partitioning(String::from("aab")),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
    }

    #[test]
    fn palindrome_partitioning_test_2() {
        assert_eq!(palindrome_partitioning(String::from("a")), vec![vec!["a"]]);
    }
}
