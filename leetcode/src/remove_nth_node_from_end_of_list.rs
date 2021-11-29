use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut pre_head = Box::new(ListNode::new(-1));
        pre_head.next = head;

        // faster is a new Box points to ListNode
        let mut faster = pre_head.clone();
        // `pre_head.clone()` clones every node in the list!!
        // println!("{:p}", faster.next.unwrap()); // will print different memory address
        // println!("{:p}", pre_head.next.unwrap()); // Need to comment out below codes to compile

        // slower is a mutable reference to ListNode: &mut ListNode
        let mut slower = pre_head.as_mut();

        for _ in 0..n {
            faster = faster.next.unwrap(); // move is OK because faster list is a totally new copy
        }

        while let Some(_) = faster.next {
            faster = faster.next.unwrap(); // move is OK
            slower = slower.next.as_mut().unwrap();
        }

        slower.next = slower.next.as_mut().unwrap().next.take();

        pre_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_node() {
        // head(node1(1)) -> node2(2) -> node(3) -> None
        let mut node2 = Box::new(ListNode::new(2));
        node2.next = Some(Box::new(ListNode::new(3)));
        let mut node1 = Box::new(ListNode::new(1));
        node1.next = Some(node2);
        let mut head = Some(node1);

        // cargo test -- --show-output

        let result = Solution::remove_nth_from_end(head.clone(), 1);
        println!("{:?}", result); // 1->2->None

        let result = Solution::remove_nth_from_end(head.clone(), 2);
        println!("{:?}", result); // 1->3->None

        let result = Solution::remove_nth_from_end(head.clone(), 3);
        println!("{:?}", result); // 2->3->None
    }
}
