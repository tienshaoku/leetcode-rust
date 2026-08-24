fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    // use i32::MAX instead of 0 to differentiate "not handled" from "number of coins"
    let mut arr = vec![i32::MAX; amount as usize + 1];
    arr[0] = 0;

    for i in 1..=amount {
        for &c in &coins {
            let c = c as usize;
            if i >= c {
                let prev = arr[i - c];
                if prev != i32::MAX {
                    arr[i] = arr[i].min(prev + 1);
                }
            }
        }
    }

    if arr[amount] == i32::MAX {
        -1
    } else {
        arr[amount]
    }
}

#[cfg(test)]
mod coin_change_test {
    use super::*;

    #[test]
    fn coin_change_test_1() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn coin_change_test_2() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }

    #[test]
    fn coin_change_test_3() {
        assert_eq!(coin_change(vec![1], 0), 0);
    }

    #[test]
    fn coin_change_test_4() {
        assert_eq!(coin_change(vec![1], 2), 2);
    }

    #[test]
    fn coin_change_test_5() {
        assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20);
    }

    #[test]
    fn coin_change_test_6() {
        assert_eq!(coin_change(vec![2, 3, 7], 29), 6);
    }
}
