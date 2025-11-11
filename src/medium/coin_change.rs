fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut table = vec![i32::MAX; amount + 1];
    table[0] = 0;

    for i in 1..=amount {
        for &c in &coins {
            let c = c as usize;
            if c <= i {
                let prev = table[i - c];
                if prev != i32::MAX {
                    table[i] = table[i].min(prev + 1);
                }
            }
        }
    }

    if table[amount] == i32::MAX {
        -1
    } else {
        table[amount]
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
}
