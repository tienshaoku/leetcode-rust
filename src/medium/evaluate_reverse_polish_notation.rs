fn evaluate_reverse_polish_notation(tokens: Vec<String>) -> i32 {
    let mut arr = Vec::new();
    for i in tokens {
        if ["+", "-", "*", "/"].contains(&i.as_str()) {
            let a = arr.pop().unwrap();
            let b = arr.pop().unwrap();

            let new = match i.as_str() {
                "+" => b + a,
                "-" => b - a,
                "*" => b * a,
                _ => b / a,
            };
            arr.push(new);
        } else {
            arr.push(i.parse::<i32>().unwrap());
        }
    }
    arr[0]
}

#[cfg(test)]
mod evaluate_reverse_polish_notation_test {
    use super::*;

    #[test]
    fn evaluate_reverse_polish_notation_test_1() {
        assert_eq!(
            evaluate_reverse_polish_notation(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
    }

    #[test]
    fn evaluate_reverse_polish_notation_test_2() {
        assert_eq!(
            evaluate_reverse_polish_notation(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
    }

    #[test]
    fn evaluate_reverse_polish_notation_test_3() {
        assert_eq!(evaluate_reverse_polish_notation(vec!["1".to_string()]), 1);
    }

    #[test]
    fn evaluate_reverse_polish_notation_test_4() {
        assert_eq!(
            evaluate_reverse_polish_notation(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }

    #[test]
    fn evaluate_reverse_polish_notation_test_5() {
        assert_eq!(
            evaluate_reverse_polish_notation(vec![
                "4".to_string(),
                "3".to_string(),
                "-".to_string()
            ]),
            1
        );
    }
}
