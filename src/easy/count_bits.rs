fn count_bits(n: i32) -> Vec<i32> {
    let mut res = vec![0; n as usize + 1];
    for i in 1..=n as usize {
        res[i] = res[i >> 1] + (i & 1) as i32;
    }
    res
}

fn count_bits_complicated(n: i32) -> Vec<i32> {
    let mut power = 1;
    let mut res = vec![0; n as usize + 1];
    for i in 1..=n as usize {
        if i == power * 2 {
            power *= 2;
            res[i] = 1;
        } else if i > power {
            res[i] = res[i - power] + res[power];
        } else {
            res[i] = 1;
        }
    }
    res
}

#[cfg(test)]
mod count_bits_test {
    use super::*;

    #[test]
    fn count_bits_test_1() {
        assert_eq!(count_bits(2), [0, 1, 1]);
    }

    #[test]
    fn count_bits_test_2() {
        assert_eq!(count_bits(5), [0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn count_bits_test_3() {
        assert_eq!(count_bits(12), [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2]);
    }
}
