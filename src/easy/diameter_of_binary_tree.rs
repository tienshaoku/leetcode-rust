use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    /// # Returns
    /// - `depth`: max depth of the current subtree
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
        if let Some(n) = root {
            let node_ref = n.borrow();
            let left_depth = traverse(node_ref.left.clone(), max);
            let right_depth = traverse(node_ref.right.clone(), max);
            *max = (*max).max(left_depth + right_depth);

            return 1 + left_depth.max(right_depth);
        }
        0
    }

    let mut max = 0;
    traverse(root, &mut max);
    max
}

#[cfg(test)]
mod diameter_of_binary_tree_test {
    use super::*;

    #[test]
    fn diameter_of_binary_tree_test_1() {
        assert_eq!(
            diameter_of_binary_tree(to_sparse_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                None,
                None
            ])),
            3
        );
    }

    #[test]
    fn diameter_of_binary_tree_test_2() {
        assert_eq!(
            diameter_of_binary_tree(to_sparse_tree(vec![Some(1), Some(2), None])),
            1
        );
    }

    #[test]
    fn diameter_of_binary_tree_test_3() {
        assert_eq!(
            diameter_of_binary_tree(to_sparse_tree(vec![
                Some(4),
                Some(-7),
                Some(-3),
                None,
                None,
                Some(-9),
                Some(-3),
                Some(9),
                Some(-7),
                Some(-4),
                None,
                Some(6),
                None,
                Some(-6),
                Some(-6),
                None,
                None,
                Some(0),
                Some(6),
                Some(5),
                None,
                Some(9),
                None,
                None,
                Some(-1),
                Some(-4),
                None,
                None,
                None,
                Some(-2),
            ])),
            8
        );
    }
}
