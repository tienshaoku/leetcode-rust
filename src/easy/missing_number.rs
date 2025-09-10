fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut sum = (n * (n + 1) / 2) as i32;
    for i in nums {
        sum -= i;
    }
    sum as i32
}

// fn missing_number_unnecessary_binary_search(mut nums: Vec<i32>) -> i32 {
//     nums.sort();

//     let (mut left, mut right) = (0, nums.len());
//     while left < right {
//         let mid = left + (right - left) / 2;

//         if nums[mid] > mid as i32 {
//             right = mid;
//         } else {
//             left = mid + 1;
//         }
//     }
//     left as i32
// }

#[cfg(test)]
mod missing_number_test {
    use super::*;

    #[test]
    fn missing_number_test_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn missing_number_test_2() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn missing_number_test_3() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn missing_number_test_4() {
        assert_eq!(missing_number(vec![0]), 1);
    }
}
