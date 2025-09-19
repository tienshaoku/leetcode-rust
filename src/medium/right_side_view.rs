use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut arr = vec![vec![root]];
    while let Some(r) = arr.pop() {
        let mut nodes = vec![];
        let mut values = vec![];
        for i in r {
            if let Some(n) = i {
                let n_ref = n.borrow();

                values.push(n_ref.val);
                nodes.push(n_ref.left.clone());
                nodes.push(n_ref.right.clone());
            }
        }
        if let Some(&v) = values.last() {
            res.push(v);
        }
        if !nodes.is_empty() {
            arr.push(nodes);
        }
    }
    res
}

#[cfg(test)]
mod right_side_view_test {
    use super::*;

    #[test]
    fn right_side_view_test_1() {
        assert_eq!(
            right_side_view(to_sparse_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                None,
                Some(5),
                None,
                Some(4)
            ])),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn right_side_view_test_2() {
        assert_eq!(
            right_side_view(to_sparse_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                None,
                Some(5),
            ])),
            vec![1, 3, 4, 5]
        );
    }

    #[test]
    fn right_side_view_test_3() {
        assert_eq!(
            right_side_view(to_sparse_tree(vec![Some(1), None, Some(3),])),
            vec![1, 3]
        );
    }

    #[test]
    fn right_side_view_test_4() {
        assert_eq!(right_side_view(to_sparse_tree(vec![])), vec![]);
    }
}
