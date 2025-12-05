fn best_time_to_buy_and_sell_stock_with_cooldown(prices: Vec<i32>) -> i32 {
    let length = prices.len();
    let mut profit = vec![0; length + 1];
    let mut cost = vec![0; length + 1];
    cost[1] = -prices[0];
    for i in 2..length + 1 {
        cost[i] = cost[i - 1].max(profit[i - 2] - prices[i - 1]);
        profit[i] = profit[i - 1].max(cost[i - 1] + prices[i - 1]);
    }
    *profit.last().unwrap()
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_with_cooldown_test {
    use super::*;

    #[test]
    fn best_time_to_buy_and_sell_stock_with_cooldown_test_1() {
        assert_eq!(
            best_time_to_buy_and_sell_stock_with_cooldown(vec![1, 2, 3, 0, 2]),
            3
        );
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_with_cooldown_test_2() {
        assert_eq!(best_time_to_buy_and_sell_stock_with_cooldown(vec![1]), 0);
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_with_cooldown_test_3() {
        assert_eq!(best_time_to_buy_and_sell_stock_with_cooldown(vec![2, 1]), 0);
    }
}
