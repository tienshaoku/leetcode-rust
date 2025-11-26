fn is_subsequence(s: String, t: String) -> bool {
    let c1: Vec<char> = s.chars().collect();
    let c2: Vec<char> = t.chars().collect();

    let (mut i, mut j) = (0, 0);
    while j < c2.len() {
        if i == c1.len() {
            return true;
        }
        if c1[i] == c2[j] {
            i += 1;
        }
        j += 1;
    }
    if i == c1.len() {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod is_subsequence_test {
    use super::*;

    #[test]
    fn is_subsequence_test_1() {
        assert_eq!(
            is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
    }

    #[test]
    fn is_subsequence_test_2() {
        assert_eq!(
            is_subsequence(String::from("axc"), String::from("ahbgdc")),
            false
        );
    }
}
