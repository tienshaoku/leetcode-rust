fn ugly_number_two(n: i32) -> i32 {
    if n < 7 {
        return n;
    }

    let n = n as usize;
    let mut res = vec![1; n];
    let mut i2 = 0;
    let mut i3 = 0;
    let mut i5 = 0;
    for i in 1..n {
        let next2 = res[i2] * 2;
        let next3 = res[i3] * 3;
        let next5 = res[i5] * 5;

        let next = next2.min(next3).min(next5);
        res[i] = next;

        if next == next2 {
            i2 += 1;
        }
        if next == next3 {
            i3 += 1;
        }
        if next == next5 {
            i5 += 1;
        }
    }
    *res.last().unwrap()
}

#[cfg(test)]
mod ugly_number_two_test {
    use super::*;

    #[test]
    fn ugly_number_two_test_1() {
        assert_eq!(ugly_number_two(10), 12);
    }

    #[test]
    fn ugly_number_two_test_2() {
        assert_eq!(ugly_number_two(1), 1);
    }

    #[test]
    fn ugly_number_two_test_3() {
        assert_eq!(ugly_number_two(11), 15);
    }
}
