use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut num = 0;
        let mut node = &head;

        while let Some(n) = node {
            num = num * 2 + n.val;
            node = &n.next;
        }
        num
    }
}
