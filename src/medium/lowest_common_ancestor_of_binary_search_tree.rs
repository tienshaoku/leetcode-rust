use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

fn lowest_common_ancestor_of_binary_search_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let (Some(r_node), Some(q_node), Some(p_node)) = (root.clone(), p.clone(), q.clone()) {
        let root_ref = r_node.borrow();
        let root_val = root_ref.val;
        let p_val = p_node.borrow().val;
        let q_val = q_node.borrow().val;

        if root_val > p_val && root_val > q_val {
            lowest_common_ancestor_of_binary_search_tree(root_ref.left.clone(), p, q)
        } else if root_val < p_val && root_val < q_val {
            lowest_common_ancestor_of_binary_search_tree(root_ref.right.clone(), p, q)
        } else {
            root
        }
    } else {
        None
    }
}

fn lowest_common_ancestor_of_binary_search_tree_complicated(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut p_arr = vec![];
    let mut q_arr = vec![];

    fn build_path_arr(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        arr: &mut Vec<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(r) = root {
            let r_ref = r.borrow();
            arr.push(Rc::clone(&r));

            if let Some(t) = target.clone() {
                match t.borrow().val.cmp(&r_ref.val) {
                    Ordering::Equal => return,
                    Ordering::Greater => build_path_arr(r_ref.right.clone(), target, arr),
                    Ordering::Less => build_path_arr(r_ref.left.clone(), target, arr),
                }
            }
        }
    }

    build_path_arr(root.clone(), p.clone(), &mut p_arr);
    build_path_arr(root.clone(), q.clone(), &mut q_arr);

    for i in p_arr.iter().rev() {
        if q_arr.contains(&i) {
            return Some(i.clone());
        }
    }
    None
}

#[cfg(test)]
mod lowest_common_ancestor_of_binary_search_tree_test {
    use super::*;

    #[test]
    fn lowest_common_ancestor_of_binary_search_tree_test_1() {
        let root = to_sparse_tree(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = root.as_ref().unwrap().borrow().left.clone();
        let q = root.as_ref().unwrap().borrow().right.clone();

        assert_eq!(
            lowest_common_ancestor_of_binary_search_tree(root.clone(), p, q),
            root.clone()
        );
    }

    #[test]
    fn lowest_common_ancestor_of_binary_search_tree_test_2() {
        let root = to_sparse_tree(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = root.as_ref().unwrap().borrow().left.clone();
        let q = p.as_ref().unwrap().borrow().right.clone();

        assert_eq!(
            lowest_common_ancestor_of_binary_search_tree(root, p.clone(), q),
            p.clone()
        );
    }

    #[test]
    fn lowest_common_ancestor_of_binary_search_tree_test_3() {
        let root = to_sparse_tree(vec![Some(2), Some(1)]);
        let q = root.as_ref().unwrap().borrow().left.clone();

        assert_eq!(
            lowest_common_ancestor_of_binary_search_tree(root.clone(), root.clone(), q),
            root.clone()
        );
    }
}
