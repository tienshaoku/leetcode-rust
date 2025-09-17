use crate::binary_tree::{to_sparse_tree, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::collections::VecDeque;

    let mut res = vec![];
    let mut arr = VecDeque::new();
    arr.push_back(vec![root]);

    while let Some(to_visits) = arr.pop_front() {
        let mut values = vec![];
        let mut nodes = vec![];
        for i in to_visits {
            if let Some(r) = i {
                let r_ref = r.borrow();
                values.push(r_ref.val);

                nodes.push(r_ref.left.clone());
                nodes.push(r_ref.right.clone());
            }
        }
        if !values.is_empty() {
            res.push(values);
        }
        if !nodes.is_empty() {
            arr.push_back(nodes);
        }
    }
    res
}

#[cfg(test)]
mod level_order_test {
    use super::*;

    #[test]
    fn level_order_test_1() {
        assert_eq!(
            level_order(to_sparse_tree(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn level_order_test_2() {
        assert_eq!(level_order(to_sparse_tree(vec![Some(1),])), vec![vec![1]]);
    }

    #[test]
    fn level_order_test_3() {
        let result: Vec<Vec<i32>> = vec![];
        assert_eq!(level_order(to_sparse_tree(vec![])), result);
    }

    #[test]
    fn level_order_test_4() {
        assert_eq!(
            level_order(to_sparse_tree(vec![
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
                Some(5)
            ])),
            vec![vec![1], vec![2, 3], vec![4, 5]]
        );
    }
}
