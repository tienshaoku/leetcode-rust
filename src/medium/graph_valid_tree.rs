fn graph_valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n: usize = n as usize;
    if n - 1 != edges.len() {
        return false;
    }

    let mut parent: Vec<usize> = (0..n).collect();
    fn find_root(i: usize, parent: &mut Vec<usize>) -> usize {
        if i == parent[i] {
            return i;
        }
        // path compression, update leaders
        let root = find_root(parent[i], parent);
        parent[i] = root;
        root
    }

    for e in edges {
        let root_1 = find_root(e[0] as usize, &mut parent);
        let root_2 = find_root(e[1] as usize, &mut parent);

        if root_1 == root_2 {
            return false;
        }
        // merge leaders
        parent[root_1] = root_2;
    }
    true
}

#[cfg(test)]
mod graph_valid_tree_test {
    use super::*;

    #[test]
    fn graph_valid_tree_test_1() {
        assert_eq!(
            graph_valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]),
            true
        );
    }

    #[test]
    fn graph_valid_tree_test_2() {
        assert_eq!(
            graph_valid_tree(
                5,
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]
            ),
            false
        );
    }

    #[test]
    fn graph_valid_tree_test_3() {
        assert_eq!(
            graph_valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]),
            false
        );
    }
}
