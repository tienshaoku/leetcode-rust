fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; digits.len()];
    let mut prop = 1;
    for i in (0..digits.len()).rev() {
        let mut sum = digits[i] as i32 + prop;
        if sum == 10 {
            sum = 0;
            prop = 1;
        } else {
            prop = 0;
        }
        res[i] = sum;
    }
    if prop == 1 {
        res.insert(0, 1);
    }
    res
}

#[cfg(test)]
mod plus_one_test {
    use super::*;

    #[test]
    fn plus_one_test_1() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn plus_one_test_2() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn plus_one_test_3() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn plus_one_test_4() {
        assert_eq!(plus_one(vec![9, 9, 9, 9, 9]), vec![1, 0, 0, 0, 0, 0]);
    }
}
