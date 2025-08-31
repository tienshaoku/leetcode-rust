fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max = heights[0];
    let mut stack = vec![0];
    let mut idx = 1;

    while idx < heights.len() {
        if stack.is_empty() || heights[idx] >= heights[*stack.last().unwrap()] {
            stack.push(idx);
            idx += 1;
        } else {
            let top = stack.pop().unwrap();
            let width = if stack.is_empty() {
                idx
            } else {
                idx - *stack.last().unwrap() - 1
            };
            max = max.max(heights[top] * width as i32);
        }
    }
    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let width = if stack.is_empty() {
            idx
        } else {
            idx - *stack.last().unwrap() - 1
        };
        max = max.max(heights[top] * width as i32);
    }
    max
}

#[cfg(test)]
mod largest_rectangle_area_test {
    use super::*;

    #[test]
    fn largest_rectangle_area_test_1() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn largest_rectangle_area_test_2() {
        assert_eq!(largest_rectangle_area(vec![4, 6]), 8);
    }

    #[test]
    fn largest_rectangle_area_test_3() {
        assert_eq!(largest_rectangle_area(vec![3, 4, 1, 1, 1, 1, 1]), 7);
    }

    #[test]
    fn largest_rectangle_area_test_4() {
        assert_eq!(largest_rectangle_area(vec![6, 6, 5, 0, 3, 4, 10]), 15);
    }

    #[test]
    fn largest_rectangle_area_test_5() {
        assert_eq!(largest_rectangle_area(vec![6, 6, 5, 5, 5, 5]), 30);
    }

    #[test]
    fn largest_rectangle_area_test_6() {
        assert_eq!(largest_rectangle_area(vec![1, 2, 3, 4, 5]), 9);
    }

    #[test]
    fn largest_rectangle_area_test_7() {
        assert_eq!(largest_rectangle_area(vec![1, 2, 2]), 4);
    }

    #[test]
    fn largest_rectangle_area_test_8() {
        assert_eq!(largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]), 6);
    }
}

// [3, 2, 1, 4, 5, 2]
