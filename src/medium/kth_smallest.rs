use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
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
    arr[k as usize - 1]
}

#[cfg(test)]
mod kth_smallest_test {
    use super::*;

    #[test]
    fn kth_smallest_test_1() {
        assert_eq!(
            kth_smallest(
                to_sparse_tree(vec![Some(3), Some(1), Some(4), None, Some(2)]),
                1
            ),
            1
        );
    }

    #[test]
    fn kth_smallest_test_2() {
        assert_eq!(
            kth_smallest(
                to_sparse_tree(vec![
                    Some(5),
                    Some(3),
                    Some(6),
                    Some(2),
                    Some(4),
                    None,
                    None,
                    Some(1),
                ]),
                3
            ),
            3
        );
    }
}
