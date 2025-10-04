fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 2;
    }

    let mut arr = vec![1, 2];
    for i in 2..(n as usize) {
        // climb_stairs(n - 1) + 1
        // climb_stairs(n - 2) + 2
        arr.push(arr[i - 1] + arr[i - 2]);
    }
    arr[n as usize - 1]
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
