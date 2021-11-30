use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }

        let mut pre_head = Box::new(ListNode::new(-1));
        let mut tail = pre_head.as_mut();

        let mut list1 = list1;
        let mut list2 = list2;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                let tmp = list1.as_ref().unwrap().next.to_owned();
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tmp;
            } else {
                let tmp = list2.as_ref().unwrap().next.to_owned();
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tmp;
            }
        }

        if list1.is_some() {
            tail.next = list1.take();
        }
        if list2.is_some() {
            tail.next = list2.take();
        }

        pre_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut node1 = ListNode::new(1);
        println!("address of node1 = {:p}", &node1);
        let node2 = ListNode::new(2);
        println!("address of node2 = {:p}", &node2);
        node1.next = Some(Box::new(node2));
        let list1 = Some(Box::new(node1)); // 1 -> 2

        let mut node3 = ListNode::new(1);
        println!("address of node3 = {:p}", &node3);
        let node4 = ListNode::new(4);
        println!("address of node4 = {:p}", &node4);
        node3.next = Some(Box::new(node4));
        let list2 = Some(Box::new(node3)); // 1 -> 4

        // cargo test -- --show-output
        let result = Solution::merge_two_lists(list1, list2);
        println!("{:?}", result); // 1 -> 1 -> 2 -> 4

        // It seems `to_owned()` cloned the data..
        println!("address of node1 in result = {:p}", &result.unwrap());
    }
}
