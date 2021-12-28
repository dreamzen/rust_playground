struct Solution;

use crate::list_node::ListNode;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut tmp = &head;
        while tmp.is_some() {
            len += 1;
            tmp = &tmp.as_ref().unwrap().next;
        }

        let mut node = head;
        for _ in 0..len / 2 {
            node = node.unwrap().next;
        }
        node
    }
}
