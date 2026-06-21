use crate::linked_list::ListNode;

fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut last = None;
        while let Some(mut n) = head {
            head = n.next;
            n.next = last;
            last = Some(n);
        }
        last
    }

    let mut p = head.as_deref();
    let mut c = 0;
    while let Some(n) = p {
        c += 1;
        p = n.next.as_deref();
    }
    if c <= 1 {
        return;
    }

    let mut second_half = {
        let mut l = &mut *head;
        for _ in 0..(c + 1) / 2 - 1 {
            l = &mut l.as_mut().unwrap().next;
        }
        l.as_mut().unwrap().next.take()
    };
    // only reversing half is enough
    second_half = reverse(second_half);

    let mut current = head.as_mut().unwrap().as_mut();
    while let Some(mut right) = second_half.take() {
        second_half = right.next.take();
        right.next = current.next.take();
        current.next = Some(right);

        if second_half.is_some() {
            current = current.next.as_mut().unwrap().next.as_mut().unwrap();
        }
    }
}

#[cfg(test)]
mod reorder_list_test {
    use super::*;
    use crate::linked_list::to_linked_list;

    #[test]
    fn reorder_list_test_1() {
        let list = [1, 2, 3, 4];
        let expected = [1, 4, 2, 3];
        let mut linked_list = to_linked_list(&list);
        reorder_list(&mut linked_list);
        assert_eq!(linked_list, to_linked_list(&expected));
    }

    #[test]
    fn reorder_list_test_2() {
        let list = [1, 2, 3, 4, 5];
        let expected = [1, 5, 2, 4, 3];
        let mut linked_list = to_linked_list(&list);
        reorder_list(&mut linked_list);
        assert_eq!(linked_list, to_linked_list(&expected));
    }
}
