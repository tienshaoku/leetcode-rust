use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node_ref = node.borrow();
        let left = max_depth(node_ref.left.clone());
        let right = max_depth(node_ref.right.clone());
        return 1 + left.max(right);
    }
    0
}

#[cfg(test)]
mod max_depth_test {
    use super::*;

    #[test]
    fn max_depth_test_1() {
        assert_eq!(
            max_depth(to_sparse_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            3
        );
    }

    #[test]
    fn max_depth_test_2() {
        assert_eq!(max_depth(to_sparse_tree(vec![Some(1), None, Some(2)])), 2);
    }
}
