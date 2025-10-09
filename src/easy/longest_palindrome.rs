fn longest_palindrome(s: String) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for i in s.chars() {
        let mut entry = map.entry(i).or_insert(0);
        *entry += 1;
    }
    let mut res = 0;
    let mut has_single = false;
    for (_, &count) in map.iter() {
        if !has_single && count % 2 != 0 {
            has_single = true;
        }
        res += count / 2 * 2;
    }
    if has_single {
        res + 1
    } else {
        res
    }
}

#[cfg(test)]
mod longest_palindrome_test {
    use super::*;

    #[test]
    fn longest_palindrome_test_1() {
        assert_eq!(longest_palindrome(String::from("abccccdd")), 7);
    }

    #[test]
    fn longest_palindrome_test_2() {
        assert_eq!(longest_palindrome(String::from("a")), 1);
    }

    #[test]
    fn longest_palindrome_test_3() {
        assert_eq!(longest_palindrome(String::from("aaabbccc")), 7);
    }

    #[test]
    fn longest_palindrome_test_4() {
        assert_eq!(longest_palindrome(String::from("aabbbbcc")), 8);
    }
}
