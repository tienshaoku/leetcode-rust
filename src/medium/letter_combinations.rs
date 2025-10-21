fn letter_combinations(digits: String) -> Vec<String> {
    fn backtrack(
        digits: &str,
        digits_to_letter: &[&str; 10],
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if current.len() == digits.len() {
            result.push(current.clone());
            return;
        }

        let digit = digits.chars().nth(current.len()).unwrap();
        let letters = digits_to_letter[digit.to_digit(10).unwrap() as usize];

        for letter in letters.chars() {
            current.push(letter);
            backtrack(digits, digits_to_letter, current, result);
            current.pop();
        }
    }

    let digit_to_letters = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];
    let mut res = vec![];
    backtrack(&digits, &digit_to_letters, &mut String::new(), &mut res);
    res
}

fn letter_combinations_without_backtracking(digits: String) -> Vec<String> {
    let digit_to_letters = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
    ];

    let mut res = vec![String::from("")];
    for i in digits.chars() {
        let letters = digit_to_letters[i.to_digit(10).unwrap() as usize]
            .chars()
            .collect::<Vec<char>>();
        let length = letters.len();
        res.reserve(length);

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

    #[test]
    fn letter_combinations_test_1() {
        assert_eq!(
            letter_combinations(String::from("23")),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }

    #[test]
    fn letter_combinations_test_2() {
        assert_eq!(letter_combinations(String::from("2")), ["a", "b", "c"]);
    }
}
