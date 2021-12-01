use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return match head {
            Some(mut h) => match h.next {
                Some(mut t) => {
                    h.next = Self::swap_pairs(t.next);
                    t.next = Some(h);
                    Some(t)
                }
                None => Some(h),
            },
            None => None,
        };
    }
}
