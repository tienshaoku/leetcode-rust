use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn to_arr(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(n) = node {
            let n_ref = n.borrow();
            to_arr(n_ref.left.clone(), arr);
            arr.push(n_ref.val);
            to_arr(n_ref.right.clone(), arr);
        }
    }

    let mut arr = vec![];
    to_arr(root, &mut arr);

    for i in 0..(arr.len() - 1) {
        if arr[i] >= arr[i + 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod is_valid_bst_test {
    use super::*;

    #[test]
    fn is_valid_bst_test_1() {
        assert_eq!(
            is_valid_bst(to_sparse_tree(vec![Some(2), Some(1), Some(3),])),
            true
        );
    }

    #[test]
    fn is_valid_bst_test_2() {
        assert_eq!(
            is_valid_bst(to_sparse_tree(vec![
                Some(5),
                Some(1),
                Some(4),
                None,
                None,
                Some(3),
                Some(6),
            ])),
            false
        );
    }

    #[test]
    fn is_valid_bst_test_3() {
        assert_eq!(is_valid_bst(to_sparse_tree(vec![Some(1)])), true);
    }

    #[test]
    fn is_valid_bst_test_4() {
        assert_eq!(
            is_valid_bst(to_sparse_tree(vec![
                Some(5),
                Some(4),
                Some(6),
                None,
                None,
                Some(3),
                Some(7),
            ])),
            false
        );
    }

    #[test]
    fn is_valid_bst_test_5() {
        assert_eq!(
            is_valid_bst(to_sparse_tree(vec![
                Some(32),
                Some(26),
                Some(47),
                Some(19),
                None,
                None,
                Some(56),
                None,
                Some(27),
            ])),
            false
        );
    }

    #[test]
    fn is_valid_bst_test_6() {
        assert_eq!(
            is_valid_bst(to_sparse_tree(vec![Some(2), Some(2), Some(2),])),
            false
        );
    }
}
