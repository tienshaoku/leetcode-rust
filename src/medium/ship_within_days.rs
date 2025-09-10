fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
    let mut max = *weights.iter().max().unwrap();
    if days == weights.len() as i32 {
        return max;
    };

    let mut sum = weights.iter().sum();
    while max < sum {
        let mid = max + (sum - max) / 2;
        let mut day_count = 1;
        let mut current = mid;
        for i in &weights {
            if *i > current {
                day_count += 1;
                current = mid;
            }
            current -= i;
        }
        if day_count <= days {
            sum = mid;
        } else {
            max = mid + 1;
        }
    }
    // answer will always the left bound
    max
}

#[cfg(test)]
mod ship_within_days_test {
    use super::*;

    #[test]
    fn ship_within_days_test_1() {
        assert_eq!(ship_within_days(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    }

    #[test]
    fn ship_within_days_test_2() {
        assert_eq!(ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    }

    #[test]
    fn ship_within_days_test_3() {
        assert_eq!(ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn ship_within_days_test_4() {
        assert_eq!(ship_within_days(vec![1, 4, 7, 11, 15], 2), 23);
    }
}
