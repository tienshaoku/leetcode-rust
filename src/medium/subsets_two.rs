use crate::vector::normalise;

fn subsets_two(nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::collections::HashSet;

    let mut set: HashSet<Vec<i32>> = HashSet::new();
    set.insert(vec![]);

    for i in nums {
        let clone = set.clone();
        for j in clone.iter() {
            let mut tmp = j.clone();
            tmp.push(i);
            tmp.sort();
            set.insert(tmp);
        }
    }
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
