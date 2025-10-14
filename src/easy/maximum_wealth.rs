fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|v| v.iter().sum()).max().unwrap()
}

#[cfg(test)]
mod maximum_wealth_test {
    use super::*;

    #[test]
    fn maximum_wealth_test_1() {
        let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
        assert_eq!(maximum_wealth(accounts), 6);
    }

    #[test]
    fn maximum_wealth_test_2() {
        let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        assert_eq!(maximum_wealth(accounts), 10);
    }

    #[test]
    fn maximum_wealth_test_3() {
        let accounts = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        assert_eq!(maximum_wealth(accounts), 17);
    }

    #[test]
    fn maximum_wealth_test_4() {
        let accounts = vec![vec![1]];
        assert_eq!(maximum_wealth(accounts), 1);
    }
}
