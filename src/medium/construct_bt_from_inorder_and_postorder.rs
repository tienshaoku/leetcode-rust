use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn construct_bt_from_inorder_and_postorder(
    inorder: Vec<i32>,
    postorder: Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..inorder.len() {
        map.insert(inorder[i], i);
    }

    fn traverse(
        inorder: &[i32],
        postorder: &[i32],
        map: &HashMap<i32, usize>,
        offset: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let val: i32 = *postorder.last().unwrap();
        let mut node = TreeNode::new(val);

        let pos = *map.get(&val).unwrap() - offset;
        let inorder_left = &inorder[0..pos];
        let inorder_right = &inorder[pos + 1..];

        let postorder_left = &postorder[0..inorder_left.len()];
        let postorder_right = &postorder[inorder_left.len()..postorder.len() - 1];

        if inorder_left.len() != 0 {
            node.left = traverse(inorder_left, postorder_left, map, offset);
        }
        if inorder_right.len() != 0 {
            node.right = traverse(inorder_right, postorder_right, map, offset + pos + 1);
        }
        Some(Rc::new(RefCell::new(node)))
    }
    traverse(&inorder, &postorder, &map, 0)
}

#[cfg(test)]
mod construct_bt_from_inorder_and_postorder_test {
    use super::*;

    #[test]
    fn construct_bt_from_inorder_and_postorder_test_1() {
        assert_eq!(
            construct_bt_from_inorder_and_postorder(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
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
    fn construct_bt_from_inorder_and_postorder_test_2() {
        assert_eq!(
            construct_bt_from_inorder_and_postorder(vec![-1], vec![-1]),
            to_sparse_tree(vec![Some(-1),])
        );
    }

    #[test]
    fn construct_bt_from_inorder_and_postorder_test_3() {
        assert_eq!(
            construct_bt_from_inorder_and_postorder(
                vec![9, 1, 10, 3, 11, 4, 8],
                vec![9, 10, 1, 11, 8, 4, 3]
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
