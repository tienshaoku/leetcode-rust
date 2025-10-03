use crate::vector::normalise;

fn subsets_two(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    let mut arr = vec![vec![]];
    for i in nums {
        let arr_clone = arr.clone();
        for j in arr_clone {
            let mut copy = j.clone();
            copy.push(i);
            copy.sort();
            arr.push(copy);
        }
    }
    let set: HashSet<Vec<i32>> = arr.into_iter().collect();
    set.into_iter().collect()
}

#[cfg(test)]
mod subsets_two_test {
    use super::*;

    #[test]
    fn subsets_two_test_1() {
        assert_eq!(
            normalise(subsets_two(vec![1, 2, 2])),
            normalise(vec![
                vec![2, 2],
                vec![1, 2, 2],
                vec![1],
                vec![],
                vec![2],
                vec![1, 2],
            ])
        );
    }

    #[test]
    fn subsets_two_test_2() {
        assert_eq!(
            normalise(subsets_two(vec![1])),
            normalise(vec![vec![1], vec![]])
        );
    }

    #[test]
    fn subsets_two_test_3() {
        assert_eq!(
            normalise(subsets_two(vec![4, 4, 4, 1, 4])),
            normalise(vec![
                vec![],
                vec![1],
                vec![1, 4],
                vec![1, 4, 4],
                vec![1, 4, 4, 4],
                vec![1, 4, 4, 4, 4],
                vec![4],
                vec![4, 4],
                vec![4, 4, 4],
                vec![4, 4, 4, 4],
            ])
        );
    }
}
