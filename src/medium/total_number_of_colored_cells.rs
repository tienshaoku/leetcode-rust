fn colored_cells(n: i32) -> i64 {
    let n = n as i64;
    1 + 2 * n * (n - 1)
}

#[cfg(test)]
mod colored_cells_test {
    use super::*;

    #[test]
    fn colored_cells_test_1() {
        assert_eq!(colored_cells(1), 1);
    }

    #[test]
    fn colored_cells_test_2() {
        assert_eq!(colored_cells(2), 5);
    }

    #[test]
    fn colored_cells_test_3() {
        assert_eq!(colored_cells(3), 13);
    }

    #[test]
    fn colored_cells_test_4() {
        assert_eq!(colored_cells(69675), 9709071901);
    }
}
