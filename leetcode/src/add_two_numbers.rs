use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1 == None {
            return l2;
        }
        if l2 == None {
            return l1;
        }
        let mut head = ListNode::new(-1);
        let mut tail = &mut head;
        let mut carry = 0;

        let mut l1 = &l1;
        let mut l2 = &l2;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut v1 = 0;
            let mut v2 = 0;
            if let Some(n1) = l1 {
                v1 = n1.val;
                l1 = &n1.next;
            }
            if let Some(n2) = l2 {
                v2 = n2.val;
                l2 = &n2.next;
            }
            let total = v1 + v2 + carry;
            carry = total / 10;
            let node = ListNode::new(total % 10);
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}
