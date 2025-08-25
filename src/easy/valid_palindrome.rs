fn is_valid_palindrome(s: String) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if cleaned.len() == 0 {
        return true;
    }

    let mut left = 0;
    let mut right = cleaned.len() - 1;
    while left < right {
        if cleaned[left] != cleaned[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

#[cfg(test)]
mod is_valid_palindrome_test {
    use super::*;

    #[test]
    fn is_valid_palindrome_test_1() {
        assert_eq!(
            is_valid_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn is_valid_palindrome_test_2() {
        assert_eq!(is_valid_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn is_valid_palindrome_test_3() {
        assert_eq!(is_valid_palindrome(" ".to_string()), true);
    }

    #[test]
    fn is_valid_palindrome_test_4() {
        assert_eq!(is_valid_palindrome(".,".to_string()), true);
    }

    #[test]
    fn is_valid_palindrome_test_5() {
        assert_eq!(is_valid_palindrome("a".to_string()), true);
    }

    #[test]
    fn is_valid_palindrome_test_6() {
        assert_eq!(is_valid_palindrome("0P".to_string()), false);
    }
}
