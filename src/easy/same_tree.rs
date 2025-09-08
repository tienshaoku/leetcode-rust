use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p_ref), Some(q_ref)) => {
            let left_node = p_ref.borrow();
            let right_node = q_ref.borrow();

            let left_result = is_same_tree(left_node.left.clone(), right_node.left.clone());
            let right_result = is_same_tree(left_node.right.clone(), right_node.right.clone());
            left_node.val == right_node.val && left_result && right_result
        }
        (None, None) => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod is_same_tree_test {
    use super::*;

    #[test]
    fn is_same_tree_test_1() {
        assert_eq!(
            is_same_tree(
                to_sparse_tree(vec![Some(1), Some(2), Some(3)]),
                to_sparse_tree(vec![Some(1), Some(2), Some(3)])
            ),
            true
        )
    }

    #[test]
    fn is_same_tree_test_2() {
        assert_eq!(
            is_same_tree(
                to_sparse_tree(vec![Some(1), Some(2), None]),
                to_sparse_tree(vec![Some(1), None, Some(2)])
            ),
            false
        )
    }

    #[test]
    fn is_same_tree_test_3() {
        assert_eq!(
            is_same_tree(
                to_sparse_tree(vec![Some(1), Some(2), Some(1)]),
                to_sparse_tree(vec![Some(1), Some(1), Some(2)])
            ),
            false
        )
    }
}
