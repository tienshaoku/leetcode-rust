use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut res = true;

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, res: &mut bool) -> i32 {
        if let Some(r) = root {
            let node = r.borrow();

            let left_depth = traverse(node.left.clone(), res);
            let right_depth = traverse(node.right.clone(), res);

            if left_depth.abs_diff(right_depth) > 1 {
                *res = false;
            }

            return 1 + left_depth.max(right_depth);
        }
        0
    }

    traverse(root, &mut res);
    res
}

#[cfg(test)]
mod is_balanced_test {
    use super::*;

    #[test]
    fn is_balanced_test_1() {
        assert_eq!(
            is_balanced(to_sparse_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            true
        );
    }

    #[test]
    fn is_balanced_test_2() {
        assert_eq!(
            is_balanced(to_sparse_tree(vec![
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3),
                None,
                None,
                Some(4),
                Some(4)
            ])),
            false
        );
    }

    #[test]
    fn is_balanced_test_3() {
        assert_eq!(is_balanced(to_sparse_tree(vec![None])), true);
    }
}
