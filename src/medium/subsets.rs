fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr = vec![vec![]];
    for i in nums {
        let arr_clone = arr.clone();
        for j in arr_clone {
            let mut copy = j.clone();
            copy.push(i);
            arr.push(copy);
        }
    }
    arr
}

#[cfg(test)]
mod subsets_test {
    use super::*;

    #[test]
    fn subsets_test_1() {
        assert_eq!(
            subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
    }

    #[test]
    fn subsets_test_2() {
        assert_eq!(subsets(vec![1]), vec![vec![], vec![1]]);
    }
}
