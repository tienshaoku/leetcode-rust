fn connected_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    let n = n as usize;
    let mut arr: Vec<usize> = (0..n).collect();
    let mut size = vec![1; n];

    fn find_root(arr: &mut Vec<usize>, target: usize) -> usize {
        if arr[target] == target {
            return target;
        }

        let mut root = target;
        while arr[root] != root {
            root = arr[root];
        }

        let mut current = target;
        while arr[current] != root {
            let tmp = arr[current];
            arr[current] = root;
            current = tmp;
        }
        root
    }

    for vec in edges {
        let f_root = find_root(&mut arr, vec[0] as usize);
        let s_root = find_root(&mut arr, vec[1] as usize);
        if f_root == s_root {
            continue;
        }

        if size[f_root] < size[s_root] {
            arr[f_root] = s_root;
            size[s_root] += size[f_root];
        } else {
            arr[s_root] = f_root;
            size[f_root] += size[s_root];
        }
    }

    let mut set = HashSet::new();
    for i in 0..n {
        let root = find_root(&mut arr, i);
        set.insert(root);
    }
    set.len() as i32
}

#[cfg(test)]
mod connected_components_test {
    use super::*;

    #[test]
    fn connected_components_test_1() {
        assert_eq!(
            connected_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]),
            2
        );
    }

    #[test]
    fn connected_components_test_2() {
        assert_eq!(
            connected_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]),
            1
        );
    }

    #[test]
    fn connected_components_test_3() {
        assert_eq!(
            connected_components(5, vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]),
            1
        );
    }

    #[test]
    fn connected_components_test_4() {
        assert_eq!(
            connected_components(4, vec![vec![0, 1], vec![2, 3], vec![1, 2]]),
            1
        );
    }

    #[test]
    fn connected_components_test_5() {
        assert_eq!(connected_components(3, vec![vec![2, 0], vec![2, 1]]), 1);
    }

    #[test]
    fn connected_components_test_6() {
        assert_eq!(
            connected_components(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![3, 5]]
            ),
            1
        );
    }

    #[test]
    fn connected_components_test_7() {
        assert_eq!(
            connected_components(
                10,
                vec![
                    vec![5, 8],
                    vec![3, 5],
                    vec![1, 9],
                    vec![4, 5],
                    vec![0, 2],
                    vec![7, 8],
                    vec![4, 9]
                ]
            ),
            3
        );
    }
}
