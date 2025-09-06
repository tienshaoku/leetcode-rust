fn length_of_longest_substring(s: String) -> i32 {
    if [0, 1].contains(&s.len()) {
        return s.len() as i32;
    }

    let mut max = 1;
    let chars: Vec<char> = s.chars().collect();
    let (mut left, mut right) = (0, 1);
    while right < s.len() {
        if s[left..right].contains(chars[right]) {
            left += s[left..right].find(chars[right]).unwrap() + 1;
        }
        max = max.max(right - left + 1);
        right += 1;
    }
    max as i32
}

#[cfg(test)]
mod length_of_longest_substring_test {
    use super::*;

    #[test]
    fn length_of_longest_substring_test_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn length_of_longest_substring_test_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn length_of_longest_substring_test_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn length_of_longest_substring_test_4() {
        assert_eq!(length_of_longest_substring("au".to_string()), 2);
    }
}
