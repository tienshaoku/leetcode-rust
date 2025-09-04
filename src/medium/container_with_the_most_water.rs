fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let (mut left, mut right) = (0, height.len() - 1);

    while left < right {
        max = max.max(height[left].min(height[right]) * (right - left) as i32);
        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }
    max
}

#[cfg(test)]
mod max_area_test {
    use super::*;

    #[test]
    fn max_area_test_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
    // 1,7 - 8,7,

    #[test]
    fn max_area_test_2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }

    #[test]
    fn max_area_test_3() {
        assert_eq!(max_area(vec![1, 2, 4, 3]), 4);
    }

    #[test]
    fn max_area_test_4() {
        assert_eq!(max_area(vec![1, 2, 3, 1000, 9]), 9);
    }

    #[test]
    fn max_area_test_5() {
        assert_eq!(max_area(vec![1, 3, 2, 5, 25, 24, 5]), 24);
    }
}
