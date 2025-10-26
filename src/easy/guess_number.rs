fn guess_number(n: i32) -> i32 {
    let (mut left, mut right) = (1, n);
    while left < right {
        let mid = left + (right - left) / 2;
        match guess(mid) {
            -1 => {
                right = mid;
            }
            1 => {
                left = mid + 1;
            }
            _ => {
                return mid;
            }
        }
    }
    left
}

fn guess(n: i32) -> i32 {
    match n {
        6 => 0,
        7.. => -1,
        _ => 1,
    }
    // match n {
    //     2 => 0,
    //     1 => 1,
    //     _ => -1,
    // }
}

#[cfg(test)]
mod guess_number_test {
    use super::*;

    #[test]
    fn guess_number_test_1() {
        assert_eq!(guess_number(10), 6);
    }

    #[test]
    fn guess_number_test_2() {
        assert_eq!(guess_number(1), 1);
    }

    #[test]
    fn guess_number_test_3() {
        assert_eq!(guess_number(2), 1);
    }

    #[test]
    fn guess_number_test_4() {
        assert_eq!(guess_number(3), 2);
    }
}
