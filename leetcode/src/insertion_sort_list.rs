use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut pre_head = ListNode::new(-5001);
        let mut head = head;

        while head.is_some() {
            // take head
            let mut to_insert = head;
            // move head ahead
            head = to_insert.as_mut().unwrap().next.take();

            // find position to insert
            let mut node = &mut pre_head;
            while node.next.is_some()
                && node.next.as_ref().unwrap().val < to_insert.as_ref().unwrap().val
            {
                node = node.next.as_mut().unwrap();
            }
            let mut tmp = None;
            if node.next.is_some() {
                tmp = node.next.take();
            }
            to_insert.as_mut().unwrap().next = tmp;
            node.next = to_insert;
        }

        pre_head.next
    }
}
