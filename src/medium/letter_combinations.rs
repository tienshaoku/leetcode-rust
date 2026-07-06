fn letter_combinations(digits: String) -> Vec<String> {
    fn backtrack(
        digits: &Vec<usize>,
        digit_to_letters: &Vec<&str>,
        start_idx: usize,
        path: &mut String,
        res: &mut Vec<String>,
    ) {
        if path.len() == digits.len() {
            res.push(path.to_string());
            return;
        }
        for i in digit_to_letters[digits[start_idx]].chars() {
            path.push(i);
            backtrack(digits, digit_to_letters, start_idx + 1, path, res);
            path.pop();
        }
    }

    let digit_to_letters = vec![
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let digits = digits
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut res = vec![];
    backtrack(&digits, &digit_to_letters, 0, &mut String::new(), &mut res);
    res
}

fn letter_combinations_without_backtracking(digits: String) -> Vec<String> {
    let digit_to_letters = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];

    let mut res = vec![String::new()];
    for i in digits.chars() {
        let letters = digit_to_letters[i.to_digit(10).unwrap() as usize]
            .chars()
            .collect::<Vec<char>>();
        let length = letters.len();

        for j in 0..res.len() {
            let base = res[j].clone();
            res[j].push(letters[0]);
            for k in 1..length {
                res.push(format!("{}{}", base, letters[k]));
            }
        }
    }
    res
}

#[cfg(test)]
mod letter_combinations_test {
    use super::*;
    use crate::vector::normalise;

    #[test]
    fn letter_combinations_test_1() {
        assert_eq!(
            normalise(letter_combinations(String::from("23"))),
            normalise(vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"])
        );
    }

    #[test]
    fn letter_combinations_test_2() {
        assert_eq!(
            normalise(letter_combinations(String::from("2"))),
            normalise(vec!["a", "b", "c"])
        );
    }
}
