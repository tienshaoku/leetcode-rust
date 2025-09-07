use crate::binary_tree::{to_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.clone() {
        let mut node_ref = node.borrow_mut();

        let left = invert_tree(node_ref.left.take());
        let right = invert_tree(node_ref.right.take());

        node_ref.left = right;
        node_ref.right = left;
    }
    root
}

#[cfg(test)]
mod invert_tree_test {
    use super::*;

    #[test]
    fn invert_tree_test_1() {
        let input = to_tree(vec![4, 2, 7, 1, 3, 6, 9]);
        assert_eq!(invert_tree(input), to_tree(vec![4, 7, 2, 9, 6, 3, 1]));
    }

    #[test]
    fn invert_tree_test_2() {
        let input = to_tree(vec![2, 1, 3]);
        assert_eq!(invert_tree(input), to_tree(vec![2, 3, 1]));
    }
}
