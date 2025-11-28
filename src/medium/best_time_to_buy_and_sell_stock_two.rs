fn best_time_to_buy_and_sell_stock_two(prices: Vec<i32>) -> i32 {
    let mut current = prices[0];
    let mut profit = 0;
    for i in 1..prices.len() {
        if current < prices[i] {
            profit += prices[i] - current;
        }
        current = prices[i];
    }
    profit
}

fn best_time_to_buy_and_sell_stock_two_dp(prices: Vec<i32>) -> i32 {
    let mut arr = vec![0; prices.len()];
    let mut current = prices[0];
    for i in 1..prices.len() {
        arr[i] = arr[i - 1].max(arr[i - 1] + prices[i] - current);
        current = prices[i];
    }
    *arr.last().unwrap()
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_two_test {
    use super::*;

    #[test]
    fn best_time_to_buy_and_sell_stock_two_test_1() {
        assert_eq!(
            best_time_to_buy_and_sell_stock_two(vec![7, 1, 5, 3, 6, 4]),
            7
        );
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_two_test_2() {
        assert_eq!(best_time_to_buy_and_sell_stock_two(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_two_test_3() {
        assert_eq!(best_time_to_buy_and_sell_stock_two(vec![1, 2, 3, 4, 5]), 4);
    }
}
