fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::Reverse;
    use std::collections::HashMap;

    let mut map = HashMap::new();
    nums.into_iter()
        .for_each(|num| *map.entry(num).or_insert(0) += 1);

    let mut vec: Vec<(i32, i32)> = map.into_iter().collect();
    vec.sort_by_key(|&(_, a)| Reverse(a));
    vec.iter().take(k as usize).map(|tuple| tuple.0).collect()
}

fn top_k_frequent_n_square_sooooo_slow(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::cmp::Reverse;

    let mut arr = Vec::new();
    for i in nums {
        // using find is O(n), while HashMap's entry is 0(1)
        if let Some((_, v)) = arr.iter_mut().find(|&&mut (k, _)| k == i) {
            *v += 1;
        } else {
            arr.push((i, 1));
        }
    }
    arr.sort_by_key(|&(_, v)| Reverse(v));
    arr.iter().take(k as usize).map(|tuple| tuple.0).collect()
}

#[cfg(test)]
mod top_k_frequent_test {
    use super::*;

    #[test]
    fn top_k_frequent_test_1() {
        assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
    }

    #[test]
    fn top_k_frequent_test_2() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn top_k_frequent_test_3() {
        assert_eq!(top_k_frequent(vec![1, -1], 1), vec![-1]);
    }

    #[test]
    fn top_k_frequent_test_4() {
        assert_eq!(top_k_frequent(vec![1, 2], 2), vec![1, 2]);
    }
}
