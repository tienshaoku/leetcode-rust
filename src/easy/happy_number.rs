fn is_happy_number(n: i32) -> bool {
    use std::collections::HashSet;

    let mut num = n;
    let mut set = HashSet::new();
    while num != 1 {
        let mut sum = 0;
        while num > 0 {
            sum += (num % 10).pow(2);
            num /= 10;
        }
        if set.contains(&sum) {
            return false;
        }
        set.insert(sum);
        num = sum;
    }
    true
}

#[cfg(test)]
mod is_happy_number_test {
    use super::*;

    #[test]
    fn test_is_happy_number_1() {
        let n = 19;
        let result = is_happy_number(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_happy_number_2() {
        let n = 191;
        let result = is_happy_number(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_happy_number_3() {
        let n = 10;
        let result = is_happy_number(n);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_happy_number_4() {
        let n = 9999998;
        let result = is_happy_number(n);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_happy_number_5() {
        let n = 9999996;
        let result = is_happy_number(n);
        assert_eq!(result, false);
    }
}
