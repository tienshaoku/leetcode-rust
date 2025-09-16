use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn lowest_common_ancestor_of_binary_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root == p || root == q {
        return root;
    }

    if let Some(r) = root.clone() {
        let r_ref = r.borrow();
        let left = lowest_common_ancestor_of_binary_tree(r_ref.left.clone(), p.clone(), q.clone());
        let right =
            lowest_common_ancestor_of_binary_tree(r_ref.right.clone(), p.clone(), q.clone());

        match (left.is_none(), right.is_none()) {
            (true, true) => None,
            (false, true) => left,
            (true, false) => right,
            _ => root,
        }
    } else {
        None
    }
}

fn lowest_common_ancestor_of_binary_tree_complicated(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn build_path_arr(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Vec<Rc<RefCell<TreeNode>>>> {
        if let Some(r) = root {
            let r_ref = r.borrow();

            if r_ref.val == target {
                Some(vec![Rc::clone(&r)])
            } else {
                let mut left = build_path_arr(r_ref.left.clone(), target);
                let mut right = build_path_arr(r_ref.right.clone(), target);

                match (left.is_some(), right.is_some()) {
                    (false, false) => None,
                    (true, false) => {
                        left.as_mut().unwrap().push(Rc::clone(&r));
                        left
                    }
                    (false, true) => {
                        right.as_mut().unwrap().push(Rc::clone(&r));
                        right
                    }
                    (true, true) => None,
                }
            }
        } else {
            None
        }
    }

    let q_arr = build_path_arr(root.clone(), q.as_ref().unwrap().borrow().val);
    let p_arr = build_path_arr(root.clone(), p.as_ref().unwrap().borrow().val);

    for i in q_arr.unwrap() {
        if p_arr.as_ref().unwrap().contains(&i) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod lowest_common_ancestor_of_binary_tree_test {
    use super::*;

    #[test]
    fn lowest_common_ancestor_of_binary_tree_test_1() {
        let root = to_sparse_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = root.as_ref().unwrap().borrow().left.clone();
        let q = root.as_ref().unwrap().borrow().right.clone();

        assert_eq!(
            lowest_common_ancestor_of_binary_tree(root.clone(), p, q),
            root.clone()
        );
    }

    #[test]
    fn lowest_common_ancestor_of_binary_tree_test_2() {
        let root = to_sparse_tree(vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let p = root.as_ref().unwrap().borrow().left.clone();
        let q = p
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .clone();

        assert_eq!(
            lowest_common_ancestor_of_binary_tree(root, p.clone(), q),
            p.clone()
        );
    }

    #[test]
    fn lowest_common_ancestor_of_binary_tree_test_3() {
        let root = to_sparse_tree(vec![Some(1), Some(2)]);
        let q = root.as_ref().unwrap().borrow().left.clone();

        assert_eq!(
            lowest_common_ancestor_of_binary_tree(root.clone(), root.clone(), q),
            root.clone()
        );
    }
}
