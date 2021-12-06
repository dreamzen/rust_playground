use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // None
        if head.is_none() {
            return head;
        }

        // a->None => None
        if head.as_ref().unwrap().next.is_none() {
            return None;
        }

        // a->b->None => a->None
        let mut len = 0;
        let mut tmp = head.as_ref();
        while let Some(n) = tmp {
            tmp = n.next.as_ref();
            len += 1;
        }

        // locate n-1 node
        let mut pre_node = head.as_mut().unwrap();
        let n = len / 2;
        for _ in 1..n {
            pre_node = pre_node.next.as_mut().unwrap();
        }

        pre_node.next = pre_node.next.take().unwrap().next;
        head
    }
}
