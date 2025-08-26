fn two_sum_two(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;

    let mut left = 0;
    let mut right = numbers.len() - 1;

    let mut res = vec![];
    while left < right {
        match (numbers[left] + numbers[right]).cmp(&target) {
            Ordering::Equal => {
                res.push(left as i32 + 1);
                res.push(right as i32 + 1);
                break;
            }
            Ordering::Less => left += 1,
            _ => right -= 1,
        }
    }
    res
}

fn two_sum__two_slow(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = vec![];
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == target {
                res.push(i as i32 + 1);
                res.push(j as i32 + 1);
            }
        }
    }
    res
}

#[cfg(test)]
mod two_sum_two_test {
    use super::*;

    #[test]
    fn two_sum_two_test_1() {
        assert_eq!(two_sum_two(vec![2, 7, 11, 15], 9), [1, 2]);
    }

    #[test]
    fn two_sum_two_test_2() {
        assert_eq!(two_sum_two(vec![2, 3, 4], 6), [1, 3]);
    }

    #[test]
    fn two_sum_two_test_3() {
        assert_eq!(two_sum_two(vec![-1, 0], -1), [1, 2]);
    }

    #[test]
    fn two_sum_two_test_4() {
        assert_eq!(two_sum_two(vec![2, 7, 11, 14, 19, 29, 35], 18), [2, 3]);
    }
}
