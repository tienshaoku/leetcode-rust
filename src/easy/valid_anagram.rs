fn valid_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_c: Vec<char> = s.chars().collect();
    s_c.sort();

    let mut t_c: Vec<char> = t.chars().collect();
    t_c.sort();

    s_c == t_c

    // if s_c != t_c {
    //     return false;
    // }
    // true
}

#[cfg(test)]
mod valid_anagram_test {
    use super::*;

    #[test]
    fn valid_anagram_test_1() {
        let s1 = "anagram";
        let s2 = "nagaram";
        assert_eq!(valid_anagram(s1.to_string(), s2.to_string()), true);
    }

    #[test]
    fn valid_anagram_test_2() {
        let s1 = "rat";
        let s2 = "car";
        assert_eq!(valid_anagram(s1.to_string(), s2.to_string()), false);
    }
}
