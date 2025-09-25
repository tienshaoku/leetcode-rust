use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn construct_bt_from_preorder_and_inorder(
    preorder: Vec<i32>,
    inorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..inorder.len() {
        map.insert(inorder[i], i);
    }

    fn traverse(
        preorder: &[i32],
        inorder: &[i32],
        map: &HashMap<i32, usize>,
        offset: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::new(preorder[0]);
        let pos = *map.get(&preorder[0]).unwrap() - offset;

        let inorder_left = &inorder[0..pos];
        let inorder_right = &inorder[pos + 1..];

        let preorder_left = &preorder[1..1 + inorder_left.len()];
        let preorder_right = &preorder[1 + inorder_left.len()..];

        if inorder_left.len() > 0 {
            root.left = traverse(preorder_left, inorder_left, map, offset);
        }
        if inorder_right.len() > 0 {
            root.right = traverse(preorder_right, inorder_right, map, offset + pos + 1);
        }

        Some(Rc::new(RefCell::new(root)))
    }
    traverse(&preorder, &inorder, &map, 0)
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
