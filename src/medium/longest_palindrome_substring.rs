fn longest_palindrome_substring(s: String) -> String {
    let length = s.len();
    let mut arr = vec![vec![false; length]; length];

    let chars = s.chars().collect::<Vec<char>>();
    let (mut left, mut right) = (0, 0);
    // arr[y][x] needs arr[y+1][x-1] first, so fill by increasing gap
    for gap in 0..length {
        for y in 0..length - gap {
            let x = y + gap;
            arr[y][x] = match gap {
                0 => true,
                1 => chars[y] == chars[x],
                _ => chars[y] == chars[x] && arr[y + 1][x - 1],
            };
            if arr[y][x] {
                left = y;
                right = x;
            }
        }
    }
    chars[left..=right].iter().collect()
}

fn longest_palindrome_substring_2_pointers(s: String) -> String {
    if s.len() == 1 {
        return s;
    }

    fn expand_around_center(chars: &[char], left: usize, right: usize) -> (usize, usize) {
        let mut l = left as i32;
        let mut r = right as i32;

        while l >= 0 && r < chars.len() as i32 && chars[l as usize] == chars[r as usize] {
            l -= 1;
            r += 1;
        }
        ((l + 1) as usize, (r - 1) as usize)
    }

    let chars: Vec<char> = s.chars().collect();
    let (mut start, mut max) = (0, 1);
    for i in 0..s.len() {
        let (left1, right1) = expand_around_center(&chars, i, i);
        let length1 = right1 + 1 - left1;

        let (left2, right2) = expand_around_center(&chars, i, i + 1);
        let length2 = right2 + 1 - left2;

        if length1 > max {
            max = length1;
            start = left1;
        }

        if length2 > max {
            max = length2;
            start = left2;
        }
    }
    chars[start..start + max].iter().collect()
}

#[cfg(test)]
mod longest_palindrome_substring_test {
    use super::*;

    #[test]
    fn longest_palindrome_substring_test_1() {
        assert_eq!(longest_palindrome_substring(String::from("babad")), "aba");
    }

    #[test]
    fn longest_palindrome_substring_test_2() {
        assert_eq!(longest_palindrome_substring(String::from("cbbd")), "bb");
    }

    #[test]
    fn longest_palindrome_substring_test_3() {
        assert_eq!(longest_palindrome_substring(String::from("a")), "a");
    }

    #[test]
    fn longest_palindrome_substring_test_4() {
        assert_eq!(longest_palindrome_substring(String::from("aaaa")), "aaaa");
    }

    #[test]
    fn longest_palindrome_substring_test_5() {
        assert_eq!(longest_palindrome_substring(String::from("abcbe")), "bcb");
    }

    #[test]
    fn longest_palindrome_substring_test_6() {
        assert_eq!(
            longest_palindrome_substring(String::from("aacabdkacaa")),
            "aca"
        );
    }
}
