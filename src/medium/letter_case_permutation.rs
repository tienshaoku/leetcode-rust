fn letter_case_permutation(s: String) -> Vec<String> {
    let mut arr: Vec<String> = vec![String::from("")];
    for i in s.chars() {
        if i.is_ascii_digit() {
            for j in arr.iter_mut() {
                j.push(i);
            }
        } else {
            let length = arr.len();
            arr.reserve(length);
            for j in 0..length {
                let base = arr[j].clone();
                arr[j].push(i.to_ascii_lowercase());
                arr.push(format!("{}{}", base, i.to_ascii_uppercase()));
            }
        }
    }
    arr
}

#[cfg(test)]
mod letter_case_permutation_test {
    use super::*;

    #[test]
    fn letter_case_permutation_test_1() {
        assert_eq!(
            letter_case_permutation(String::from("a1b2")),
            vec!["a1b2", "A1b2", "a1B2", "A1B2"]
        );
    }

    #[test]
    fn letter_case_permutation_test_2() {
        assert_eq!(
            letter_case_permutation(String::from("3z4")),
            vec!["3z4", "3Z4"]
        );
    }
}
