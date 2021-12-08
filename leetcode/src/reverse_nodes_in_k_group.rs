use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k <= 1 {
            return head;
        }

        let mut len = 0;
        let mut tmp = &head; // use `borrow` because no update is needed
        while tmp.is_some() {
            len += 1;
            tmp = &tmp.as_ref().unwrap().next;
            if len >= k {
                break;
            }
        }
        if len < k {
            return head;
        }

        let mut head = head;
        // cannot declare tail here due to borrow checker, we can't create 2 mutable ref to head
        // let mut new_tail = head.as_mut().unwrap();
        let mut next_head = head.as_mut().unwrap().next.take();
        let mut new_head = head.take();
        for _ in 1..k {
            let mut tmp_next_head = next_head.as_mut().unwrap().next.take();
            next_head.as_mut().unwrap().next = new_head.take();
            new_head = next_head.take();
            next_head = tmp_next_head;
        }

        // find the tail again
        let mut tail = new_head.as_mut().unwrap();
        for _ in 1..k {
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = Self::reverse_k_group(next_head, k);

        new_head
    }
}
