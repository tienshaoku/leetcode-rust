use crate::linked_list::ListNode;

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(n1), Some(n2)) => {
            if n1.val >= n2.val {
                Some(Box::new(ListNode {
                    val: n2.val,
                    next: merge_two_lists(Some(n1), n2.next),
                }))
            } else {
                Some(Box::new(ListNode {
                    val: n1.val,
                    next: merge_two_lists(n1.next, Some(n2)),
                }))
            }
        }
    }
}

#[cfg(test)]
mod merge_two_lists_test {
    use super::*;
    use crate::linked_list::to_list;

    #[test]
    fn merge_two_lists_test_1() {
        let list1 = [1, 2, 4];
        let list2 = [1, 3, 4];
        let expected = [1, 1, 2, 3, 4, 4];
        assert_eq!(
            merge_two_lists(to_list(&list1), to_list(&list2)),
            to_list(&expected)
        );
    }

    #[test]
    fn merge_two_lists_test_2() {
        let list1 = [];
        let list2 = [0];
        let expected = [0];
        assert_eq!(
            merge_two_lists(to_list(&list1), to_list(&list2)),
            to_list(&expected)
        );
    }

    #[test]
    fn merge_two_lists_test_3() {
        let list1 = [];
        let list2 = [];
        let expected = [];
        assert_eq!(
            merge_two_lists(to_list(&list1), to_list(&list2)),
            to_list(&expected)
        );
    }
}
