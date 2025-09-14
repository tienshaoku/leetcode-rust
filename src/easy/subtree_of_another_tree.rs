use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    // compares the two entire trees, not just val, left & right
    if root == sub_root {
        return true;
    }

    if let Some(r) = root {
        let r_ref = r.borrow();
        let left = is_subtree(r_ref.left.clone(), sub_root.clone());
        let right = is_subtree(r_ref.right.clone(), sub_root.clone());
        return left || right;
    }
    false
}

#[cfg(test)]
mod is_subtree_test {
    use super::*;

    #[test]
    fn is_subtree_test_1() {
        assert_eq!(
            is_subtree(
                to_sparse_tree(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]),
                to_sparse_tree(vec![Some(4), Some(1), Some(2)])
            ),
            true
        );
    }

    #[test]
    fn is_subtree_test_2() {
        assert_eq!(
            is_subtree(
                to_sparse_tree(vec![
                    Some(3),
                    Some(4),
                    Some(5),
                    Some(1),
                    Some(2),
                    None,
                    None,
                    None,
                    None,
                    Some(0),
                    None
                ]),
                to_sparse_tree(vec![Some(4), Some(1), Some(2)])
            ),
            false
        );
    }
}
