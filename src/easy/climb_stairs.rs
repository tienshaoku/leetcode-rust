fn climb_stairs(n: i32) -> i32 {
    let n = n as usize;
    let mut arr = vec![1; n + 1];
    for i in 2..n + 1 {
        arr[i] = arr[i - 1] + arr[i - 2];
    }
    arr[n]
}

#[cfg(test)]
mod climb_stairs_test {
    use super::*;

    #[test]
    fn climb_stairs_test_1() {
        assert_eq!(climb_stairs(2), 2);
    }

    #[test]
    fn climb_stairs_test_2() {
        assert_eq!(climb_stairs(3), 3);
    }

    #[test]
    fn climb_stairs_test_3() {
        assert_eq!(climb_stairs(4), 5);
    }

    #[test]
    fn climb_stairs_test_4() {
        assert_eq!(climb_stairs(5), 8);
    }
}
