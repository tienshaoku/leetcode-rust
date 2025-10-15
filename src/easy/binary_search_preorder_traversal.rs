use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn binary_search_preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn traverse(node: Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(n) = node {
            let node_ref = n.borrow();
            arr.push(node_ref.val);
            traverse(node_ref.left.clone(), arr);
            traverse(node_ref.right.clone(), arr);
        }
    }
    let mut res = vec![];
    traverse(root.clone(), &mut res);
    res
}

#[cfg(test)]
mod binary_search_preorder_traversal_test {
    use super::*;

    #[test]
    fn binary_search_preorder_traversal_test_1() {
        assert_eq!(
            binary_search_preorder_traversal(to_sparse_tree(vec![
                Some(1),
                None,
                Some(2),
                None,
                Some(3)
            ])),
            [1, 2, 3]
        );
    }

    #[test]
    fn binary_search_preorder_traversal_test_2() {
        assert_eq!(
            binary_search_preorder_traversal(to_sparse_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                None,
                Some(8),
                None,
                None,
                Some(6),
                Some(7),
                Some(9),
            ])),
            [1, 2, 4, 5, 6, 7, 3, 8, 9]
        );
    }

    #[test]
    fn binary_search_preorder_traversal_test_3() {
        assert_eq!(binary_search_preorder_traversal(to_sparse_tree(vec![])), []);
    }

    #[test]
    fn binary_search_preorder_traversal_test_4() {
        assert_eq!(
            binary_search_preorder_traversal(to_sparse_tree(vec![Some(1)])),
            [1]
        );
    }
}
