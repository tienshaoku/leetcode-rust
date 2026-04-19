use crate::linked_list::ListNode;

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut last = None;
    let mut current = head;
    while let Some(mut node) = current {
        current = node.next;
        node.next = last;
        last = Some(node);
    }
    last
}

#[cfg(test)]
mod reverse_list_test {
    use super::*;
    use crate::linked_list::to_list;

    #[test]
    fn reverse_list_test_1() {
        let list = [1, 2, 4];
        let expected = [4, 2, 1];
        assert_eq!(reverse_list(to_list(&list)), to_list(&expected));
    }

    #[test]
    fn reverse_list_test_2() {
        let list = [];
        let expected = [];
        assert_eq!(reverse_list(to_list(&list)), to_list(&expected));
    }

    #[test]
    fn reverse_list_test_3() {
        let list = [1, 2];
        let expected = [2, 1];
        assert_eq!(reverse_list(to_list(&list)), to_list(&expected));
    }
}
