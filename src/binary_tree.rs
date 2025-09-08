use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_full_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
    let mut queue = vec![Rc::clone(&root)];
    let mut i = 1;
    while i < nums.len() && queue.len() > 0 {
        let mut current = queue.remove(0);
        if i < nums.len() {
            let left = Rc::new(RefCell::new(TreeNode::new(nums[i])));
            current.borrow_mut().left = Some(Rc::clone(&left));
            queue.push(left);
            i += 1;
        }

        if i < nums.len() {
            let right = Rc::new(RefCell::new(TreeNode::new(nums[i])));
            current.borrow_mut().right = Some(Rc::clone(&right));
            queue.push(right);
            i += 1;
        }
    }
    Some(root)
}

pub fn to_sparse_tree(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() || nums[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0]?)));
    let mut queue = vec![Rc::clone(&root)];
    let mut i = 1;
    while i < nums.len() && queue.len() > 0 {
        let mut current = queue.remove(0);
        if i < nums.len() {
            if let Some(n) = nums[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(n)));
                current.borrow_mut().left = Some(Rc::clone(&left));
                queue.push(left);
            };
            i += 1;
        }

        if i < nums.len() {
            if let Some(n) = nums[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(n)));
                current.borrow_mut().right = Some(Rc::clone(&right));
                queue.push(right);
            };
            i += 1;
        }
    }
    Some(root)
}
