fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut res = vec![0; cost.len()];
    for i in (0..cost.len()).rev() {
        if i >= cost.len() - 2 {
            res[i] = cost[i];
            continue;
        }
        res[i] = cost[i] + res[i + 1].min(res[i + 2]);
    }
    res[0].min(res[1])
}

#[cfg(test)]
mod min_cost_climbing_stairs_test {
    use super::*;

    #[test]
    fn min_cost_climbing_stairs_test_1() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }

    #[test]
    fn min_cost_climbing_stairs_test_2() {
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }

    #[test]
    fn min_cost_climbing_stairs_test_3() {
        assert_eq!(min_cost_climbing_stairs(vec![1, 2, 7, 3, 12, 4, 6]), 9);
    }

    #[test]
    fn min_cost_climbing_stairs_test_4() {
        assert_eq!(min_cost_climbing_stairs(vec![0, 1, 1, 1]), 1);
    }
}
