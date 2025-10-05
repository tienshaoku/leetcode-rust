fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut arr = vec![0; cost.len() + 1];
    arr[cost.len() - 1] = cost[cost.len() - 1];

    for i in (0..=(cost.len() - 2)).rev() {
        arr[i] = cost[i] + arr[i + 1].min(arr[i + 2]);
    }
    arr[0].min(arr[1])
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
