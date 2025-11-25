fn edit_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    // words can be empty string
    let mut arr = vec![vec![0; m + 1]; n + 1];

    for i in 0..=m {
        arr[0][i] = i as i32;
    }
    for j in 0..=n {
        arr[j][0] = j as i32;
    }

    let c1: Vec<char> = word1.chars().collect();
    let c2: Vec<char> = word2.chars().collect();
    for i in 1..=n {
        for j in 1..=m {
            if c1[j - 1] == c2[i - 1] {
                arr[i][j] = arr[i - 1][j - 1];
            } else {
                arr[i][j] = 1 + arr[i - 1][j - 1].min(arr[i][j - 1]).min(arr[i - 1][j]);
            }
        }
    }
    arr[n][m]
}

#[cfg(test)]
mod edit_distance_test {
    use super::*;

    #[test]
    fn edit_distance_test_1() {
        assert_eq!(edit_distance(String::from("horse"), String::from("ros")), 3);
    }

    #[test]
    fn edit_distance_test_2() {
        assert_eq!(
            edit_distance(String::from("intention"), String::from("execution")),
            5
        );
    }
}
