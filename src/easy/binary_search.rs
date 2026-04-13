fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    // we use exclusive right range [l, r)
    let (mut l, mut r) = (0, nums.len());
    let mut mid;
    while l < r {
        mid = l + (r - l) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target {
            // new range: [l, r'), where r' = mid
            // this fits the original range [l, r) where r was exclusive,
            // and here as we know the answer lies in l, l+1... mid-1 (exclusive r')
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    -1
}

#[cfg(test)]
mod binary_search_test {
    use super::*;

    #[test]
    fn binary_search_test_1() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn binary_search_test_2() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn binary_search_test_3() {
        assert_eq!(binary_search(vec![2], 2), 0);
    }

    #[test]
    fn binary_search_test_4() {
        assert_eq!(binary_search(vec![3], 2), -1);
    }

    #[test]
    fn binary_search_test_5() {
        assert_eq!(binary_search(vec![-1, 0, 3, 5, 9, 12], -1), 0);
    }

    #[test]
    fn binary_search_test_6() {
        assert_eq!(binary_search(vec![2, 5], 5), 1);
    }
}
