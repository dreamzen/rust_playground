use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut odd_pre_head = ListNode::new(0);
        let mut even_pre_head = ListNode::new(0);

        let mut odd_tail = &mut odd_pre_head;
        let mut even_tail = &mut even_pre_head;

        let mut odd = true;

        let mut head = head;
        while let Some(mut h) = head {
            head = h.next.take();
            if odd {
                odd_tail.next = Some(h);
                odd_tail = odd_tail.next.as_mut().unwrap();
            } else {
                even_tail.next = Some(h);
                even_tail = even_tail.next.as_mut().unwrap();
            }
            odd = !odd;
        }
        odd_tail.next = even_pre_head.next;

        odd_pre_head.next
    }
}
