fn is_valid(s: String) -> bool {
    let mut arr = Vec::new();

    for i in s.chars() {
        match i {
            '(' => arr.push('('),
            '[' => arr.push('['),
            '{' => arr.push('{'),
            ')' => {
                if arr.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if arr.pop() != Some('[') {
                    return false;
                }
            }
            _ => {
                if arr.pop() != Some('{') {
                    return false;
                }
            }
        }
    }
    arr.is_empty()
}

#[cfg(test)]
mod valid_parentheses_test {
    use super::*;

    #[test]
    fn test_is_valid_1() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_is_valid_2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_is_valid_3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_is_valid_4() {
        assert_eq!(is_valid("([])".to_string()), true);
    }

    #[test]
    fn test_is_valid_5() {
        assert_eq!(is_valid("])([".to_string()), false);
    }

    #[test]
    fn test_is_valid_6() {
        assert_eq!(is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_is_valid_7() {
        assert_eq!(is_valid("[".to_string()), false);
    }
}
