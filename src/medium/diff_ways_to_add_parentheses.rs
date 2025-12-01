fn diff_ways_to_add_parentheses(expression: String) -> Vec<i32> {
    let mut res = vec![];
    for (i, c) in expression.chars().enumerate() {
        if c == '+' || c == '-' || c == '*' {
            let s1 = diff_ways_to_add_parentheses(expression[0..i].to_string());
            let s2 = diff_ways_to_add_parentheses(expression[i + 1..].to_string());
            for a in &s1 {
                for b in &s2 {
                    match c {
                        '+' => res.push(a + b),
                        '-' => res.push(a - b),
                        _ => res.push(a * b),
                    }
                }
            }
        }
    }
    if res.is_empty() {
        res.push(expression.parse().unwrap());
    }
    res
}

#[cfg(test)]
mod diff_ways_to_add_parentheses_test {
    use super::*;

    #[test]
    fn diff_ways_to_add_parentheses_test_1() {
        assert_eq!(diff_ways_to_add_parentheses(String::from("2-1-1")), [2, 0]);
    }

    #[test]
    fn diff_ways_to_add_parentheses_test_2() {
        assert_eq!(
            diff_ways_to_add_parentheses(String::from("2*3-4*5")),
            [-34, -10, -14, -10, 10]
        );
    }
}
