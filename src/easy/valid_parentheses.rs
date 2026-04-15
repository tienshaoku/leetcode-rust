fn is_valid_parentheses(s: String) -> bool {
    fn check(i: char) -> char {
        match i {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => unreachable!(),
        }
    }

    let mut arr = vec![];
    for i in s.chars() {
        match i {
            '(' | '[' | '{' => arr.push(i),
            ')' | ']' | '}' => {
                if arr.pop() != Some(check(i)) {
                    return false;
                }
            }
            _ => return false,
        }
    }
    arr.is_empty()
}

#[cfg(test)]
mod is_valid_parentheses_test {
    use super::*;

    #[test]
    fn test_is_valid_parentheses_1() {
        assert_eq!(is_valid_parentheses("()".to_string()), true);
    }

    #[test]
    fn test_is_valid_parentheses_2() {
        assert_eq!(is_valid_parentheses("()[]{}".to_string()), true);
    }

    #[test]
    fn test_is_valid_parentheses_3() {
        assert_eq!(is_valid_parentheses("(]".to_string()), false);
    }

    #[test]
    fn test_is_valid_parentheses_4() {
        assert_eq!(is_valid_parentheses("([])".to_string()), true);
    }

    #[test]
    fn test_is_valid_parentheses_5() {
        assert_eq!(is_valid_parentheses("])([".to_string()), false);
    }

    #[test]
    fn test_is_valid_parentheses_6() {
        assert_eq!(is_valid_parentheses("([)]".to_string()), false);
    }

    #[test]
    fn test_is_valid_parentheses_7() {
        assert_eq!(is_valid_parentheses("[".to_string()), false);
    }
}
