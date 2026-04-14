fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
    piles.sort();
    let last = *piles.last().unwrap();
    if h == piles.len() as i32 {
        return last;
    }
    let (mut l, mut r) = (1, last);
    let mut m;
    while l < r {
        m = l + (r - l) / 2;
        // ceiling(v/m) = (v+m-1)/m, cuz (m-1)/m isn't enough to add exactly 1, but enough to propagate
        let sum: i32 = piles.iter().map(|&v| (v + m - 1) / m).sum();
        if sum > h {
            l = m + 1;
        } else {
            r = m;
        }
    }
    // the first/smallest possible answer
    l
}

#[cfg(test)]
mod min_eating_speed_test {
    use super::*;

    #[test]
    fn min_eating_speed_test_1() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn min_eating_speed_test_2() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn min_eating_speed_test_3() {
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    fn min_eating_speed_test_4() {
        assert_eq!(min_eating_speed(vec![312884470], 312884469), 2);
    }

    #[test]
    fn min_eating_speed_test_5() {
        assert_eq!(min_eating_speed(vec![1000000000], 2), 500000000);
    }

    #[test]
    fn min_eating_speed_test_6() {
        assert_eq!(
            min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
            3
        );
    }

    #[test]
    fn min_eating_speed_test_7() {
        assert_eq!(min_eating_speed(vec![15, 30], 3), 15);
    }
}
