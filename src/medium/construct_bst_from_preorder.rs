use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn construct_bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(preorder[0]);
    if preorder.len() == 0 {
        return None;
    }
    if preorder.len() != 1 {
        let mut pos = 0;
        for i in 0..preorder.len() {
            if preorder[i] > preorder[0] {
                pos = i;
                break;
            }
        }
        let mut left;
        let mut right;
        match pos {
            0 => {
                left = preorder[1..].to_vec();
                right = vec![];
            }
            1 => {
                left = vec![];
                right = preorder[1..].to_vec();
            }
            _ => {
                left = preorder[1..pos].to_vec();
                right = preorder[pos..].to_vec();
            }
        }

        if !left.is_empty() {
            root.left = construct_bst_from_preorder(left);
        }
        if !right.is_empty() {
            root.right = construct_bst_from_preorder(right);
        }
    }

    Some(Rc::new(RefCell::new(root)))
}

//        3
//   1        4
// 9   10  11   8

// pre:  [3, 1, 9, 10, 4, 11, 8]
// in:   [9, 1, 10, 3, 11, 4, 8]

#[cfg(test)]
mod construct_bst_from_preorder_test {
    use super::*;

    #[test]
    fn construct_bst_from_preorder_test_1() {
        assert_eq!(
            construct_bst_from_preorder(vec![8, 5, 1, 7, 10, 12]),
            to_sparse_tree(vec![
                Some(8),
                Some(5),
                Some(10),
                Some(1),
                Some(7),
                None,
                Some(12),
            ])
        );
    }

    #[test]
    fn construct_bst_from_preorder_test_2() {
        assert_eq!(
            construct_bst_from_preorder(vec![1, 3]),
            to_sparse_tree(vec![Some(1), None, Some(3)])
        );
    }

    #[test]
    fn construct_bst_from_preorder_test_3() {
        assert_eq!(
            construct_bst_from_preorder(vec![8, 4, 1, 9, 12]),
            to_sparse_tree(vec![
                Some(8),
                Some(4),
                Some(9),
                Some(1),
                None,
                None,
                Some(12),
            ])
        );
    }

    #[test]
    fn construct_bst_from_preorder_test_4() {
        assert_eq!(
            construct_bst_from_preorder(vec![4, 2]),
            to_sparse_tree(vec![Some(4), Some(2)])
        );
    }
}
