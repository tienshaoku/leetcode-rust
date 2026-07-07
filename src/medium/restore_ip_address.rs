fn restore_ip_addresses(s: String) -> Vec<String> {
    fn is_valid(segment: &str) -> bool {
        if segment.starts_with('0') && segment.len() > 1 {
            return false;
        }
        match segment.parse::<u32>() {
            Ok(n) => n <= 255,
            Err(_) => false,
        }
    }

    fn backtrack(s: &str, start_idx: usize, path: &mut Vec<String>, res: &mut Vec<String>) {
        // when path.len() is 4, s might have or not been exhausted
        if path.len() == 4 {
            if start_idx == s.len() {
                res.push(path.join("."));
            }
            return;
        }

        // try every valid length for the next segment
        for len in 1..=3 {
            if start_idx + len > s.len() {
                break;
            }
            let segment = &s[start_idx..start_idx + len];
            if !is_valid(segment) {
                continue;
            }

            path.push(segment.to_string());
            backtrack(s, start_idx + len, path, res);
            path.pop();
        }
    }

    let mut res = vec![];
    backtrack(&s, 0, &mut vec![], &mut res);
    res
}

#[cfg(test)]
mod restore_ip_addresses_test {
    use super::*;
    use crate::vector::normalise;

    #[test]
    fn restore_ip_addresses_test_1() {
        assert_eq!(
            normalise(restore_ip_addresses(String::from("25525511135"))),
            normalise(vec!["255.255.11.135", "255.255.111.35"])
        );
    }

    #[test]
    fn restore_ip_addresses_test_2() {
        assert_eq!(
            normalise(restore_ip_addresses(String::from("0000"))),
            normalise(vec!["0.0.0.0"])
        );
    }

    #[test]
    fn restore_ip_addresses_test_3() {
        assert_eq!(
            normalise(restore_ip_addresses(String::from("101023"))),
            normalise(vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ])
        );
    }
}
