fn palindromic_substrings(s: String) -> i32 {
    let length = s.len();
    let mut arr = vec![vec![false; length]; length];
    let chars = s.chars().collect::<Vec<char>>();
    let mut res = 0;

    for gap in 0..length {
        for y in 0..length - gap {
            let x = y + gap;
            arr[y][x] = match gap {
                0 => true,
                1 => chars[y] == chars[x],
                _ => chars[y] == chars[x] && arr[y + 1][x - 1],
            };
            if arr[y][x] {
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod palindromic_substrings_test {
    use super::*;

    #[test]
    fn palindromic_substrings_test_1() {
        assert_eq!(palindromic_substrings(String::from("abc")), 3);
    }

    #[test]
    fn palindromic_substrings_test_2() {
        assert_eq!(palindromic_substrings(String::from("aaa")), 6);
    }

    #[test]
    fn palindromic_substrings_test_3() {
        assert_eq!(palindromic_substrings(String::from("b")), 1);
    }
}
