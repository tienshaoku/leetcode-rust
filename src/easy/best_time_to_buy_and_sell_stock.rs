fn best_time_to_buy_and_sell_stock(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min = prices[0];
    for i in 1..prices.len() {
        if prices[i] < min {
            min = prices[i];
        }
        profit = profit.max(prices[i] - min);
    }
    profit
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_test {
    use super::*;

    #[test]
    fn best_time_to_buy_and_sell_stock_test_1() {
        assert_eq!(best_time_to_buy_and_sell_stock(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_test_2() {
        assert_eq!(best_time_to_buy_and_sell_stock(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_test_3() {
        assert_eq!(best_time_to_buy_and_sell_stock(vec![1, 2]), 1);
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_test_4() {
        assert_eq!(best_time_to_buy_and_sell_stock(vec![2, 1, 4]), 3);
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_test_5() {
        assert_eq!(
            best_time_to_buy_and_sell_stock(vec![2, 1, 2, 1, 0, 1, 2]),
            2
        );
    }

    #[test]
    fn best_time_to_buy_and_sell_stock_test_6() {
        assert_eq!(
            best_time_to_buy_and_sell_stock(vec![4, 3, 10, 1, 6, 15, 2]),
            14
        );
    }
}
