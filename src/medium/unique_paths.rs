fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut arr = vec![vec![0; n]; m];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m - 1 || j == n - 1 {
                arr[i][j] = 1;
            } else {
                arr[i][j] = arr[i + 1][j] + arr[i][j + 1];
            }
        }
    }
    arr[0][0]
}

#[cfg(test)]
mod unique_paths_test {
    use super::*;

    #[test]
    fn unique_paths_test_1() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn unique_paths_test_2() {
        assert_eq!(unique_paths(3, 2), 3);
    }
}
