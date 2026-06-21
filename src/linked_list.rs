#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn to_linked_list(nums: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for num in nums.iter().rev() {
        let mut node = ListNode::new(*num);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}
