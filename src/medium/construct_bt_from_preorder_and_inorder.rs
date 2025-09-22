use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn construct_bt_from_preorder_and_inorder(
    preorder: Vec<i32>,
    inorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(preorder[0]);
    let index = inorder.iter().position(|&x| x == preorder[0]).unwrap();

    let inorder_left = inorder[0..index].to_vec();
    let inorder_right = inorder[index + 1..].to_vec();

    let preorder_left = preorder[1..1 + inorder_left.len()].to_vec();
    let preorder_right = preorder[1 + inorder_left.len()..].to_vec();

    if inorder_left.len() > 0 {
        root.left = construct_bt_from_preorder_and_inorder(preorder_left, inorder_left);
    }
    if inorder_right.len() > 0 {
        root.right = construct_bt_from_preorder_and_inorder(preorder_right, inorder_right);
    }

    Some(Rc::new(RefCell::new(root)))
}

//        3
//   1        4
// 9   10  11   8

// pre:  [3, 1, 9, 10, 4, 11, 8]
// in:   [9, 1, 10, 3, 11, 4, 8]

#[cfg(test)]
mod construct_bt_from_preorder_and_inorder_test {
    use super::*;

    #[test]
    fn construct_bt_from_preorder_and_inorder_test_1() {
        assert_eq!(
            construct_bt_from_preorder_and_inorder(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            to_sparse_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])
        );
    }

    #[test]
    fn construct_bt_from_preorder_and_inorder_test_2() {
        assert_eq!(
            construct_bt_from_preorder_and_inorder(vec![-1], vec![-1]),
            to_sparse_tree(vec![Some(-1),])
        );
    }

    #[test]
    fn construct_bt_from_preorder_and_inorder_test_3() {
        assert_eq!(
            construct_bt_from_preorder_and_inorder(
                vec![3, 1, 9, 10, 4, 11, 8],
                vec![9, 1, 10, 3, 11, 4, 8]
            ),
            to_sparse_tree(vec![
                Some(3),
                Some(1),
                Some(4),
                Some(9),
                Some(10),
                Some(11),
                Some(8)
            ])
        );
    }
}
