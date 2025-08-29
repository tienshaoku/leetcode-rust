fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let (mut all, mut others) = (1, 0);
    for &i in nums.iter() {
        if others != 0 {
            others *= i;
        }
        if i == 0 {
            others = all;
        }
        all *= i;
    }

    nums.into_iter()
        .map(|i| if i == 0 { others } else { all / i })
        .collect()
}

#[cfg(test)]
mod product_except_self_test {
    use super::*;

    #[test]
    fn product_except_self_test_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn product_except_self_test_2() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, 3, 4]),
            vec![0, 0, -12, 0, 0]
        );
    }

    #[test]
    fn product_except_self_test_3() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, 0, -3, 3]),
            vec![0, 0, 0, 0, 0, 0]
        );
    }
}
