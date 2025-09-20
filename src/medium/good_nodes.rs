use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if let Some(r) = root {
            let r_ref = r.borrow();
            let new_max = max.max(r_ref.val);
            let add = if r_ref.val >= new_max { 1 } else { 0 };

            add + dfs(r_ref.left.clone(), new_max) + dfs(r_ref.right.clone(), new_max)
        } else {
            0
        }
    }

    dfs(root, i32::MIN)
}

fn good_nodes_slow(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, res: &mut i32) -> bool {
        if let Some(r) = root {
            let r_ref = r.borrow();
            if r_ref.val >= *path.iter().max().unwrap() {
                *res += 1;
            }

            path.push(r_ref.val);
            let left = traverse(r_ref.left.clone(), path, res);
            let right = traverse(r_ref.right.clone(), path, res);
            if left == false && right == false {
                path.pop();
                return false;
            }
            return true;
        }
        false
    }

    traverse(
        root.clone(),
        &mut vec![root.unwrap().borrow().val],
        &mut res,
    );
    res
}

#[cfg(test)]
mod good_nodes_test {
    use super::*;

    #[test]
    fn good_nodes_test_1() {
        assert_eq!(
            good_nodes(to_sparse_tree(vec![
                Some(3),
                Some(1),
                Some(3),
                Some(3),
                None,
                Some(1),
                Some(5)
            ])),
            4
        );
    }

    #[test]
    fn good_nodes_test_2() {
        assert_eq!(
            good_nodes(to_sparse_tree(vec![
                Some(3),
                Some(3),
                None,
                Some(4),
                Some(2),
            ])),
            3
        );
    }

    #[test]
    fn good_nodes_test_3() {
        assert_eq!(good_nodes(to_sparse_tree(vec![Some(1)])), 1);
    }
}
