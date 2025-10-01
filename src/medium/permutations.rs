fn permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 1 {
        return vec![nums];
    }
    let mut res = vec![];
    for i in 0..nums.len() {
        let mut removed = nums.clone();
        removed.remove(i);
        let mut subsets = permutations(removed);

        for j in subsets.iter_mut() {
            j.push(nums[i]);
            res.push(j.to_owned());
        }
    }
    res
}

#[cfg(test)]
mod permutations_test {
    use super::*;

    #[test]
    fn permutations_test_1() {
        assert_eq!(
            permutations(vec![1, 2, 3]),
            [
                [3, 2, 1],
                [2, 3, 1],
                [3, 1, 2],
                [1, 3, 2],
                [2, 1, 3],
                [1, 2, 3],
            ]
        );
    }

    #[test]
    fn permutations_test_2() {
        assert_eq!(permutations(vec![0, 1]), [[1, 0], [0, 1],]);
    }

    #[test]
    fn permutations_test_3() {
        assert_eq!(permutations(vec![1]), [[1]]);
    }
}
