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

        let mut pre_head = ListNode::new(-1);
        let mut tail = &mut pre_head;

        let mut list1 = list1;
        let mut list2 = list2;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                // old version: `to_owned()` will clone the data from borrowed data
                // let tmp = list1.as_ref().unwrap().next.to_owned();
                // tail.next = list1.take();
                // tail = tail.next.as_mut().unwrap();
                // list1 = tmp;

                // better version: without data clone
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                // old version: `to_owned()` will clone the data from borrowed data
                // let tmp = list2.as_ref().unwrap().next.to_owned();
                // tail.next = list2.take();
                // tail = tail.next.as_mut().unwrap();
                // list2 = tmp;

                // better version: without data clone
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
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
        println!("address of node1 on stack = {:p}", &node1);
        let node2 = ListNode::new(2);
        println!("address of node2 on stack = {:p}", &node2);
        node1.next = Some(Box::new(node2));
        let list1 = Some(Box::new(node1)); // 1 -> 2

        let mut node3 = ListNode::new(1);
        println!("address of node3 on stack = {:p}", &node3);
        let node4 = ListNode::new(4);
        println!("address of node4 on stack = {:p}", &node4);
        node3.next = Some(Box::new(node4));
        let list2 = Some(Box::new(node3)); // 1 -> 4

        println!(
            "address of list1's node1 on heap = {:p}",
            &**list1.as_ref().unwrap()
        );
        println!(
            "address of list1's node2 on heap = {:p}",
            &**list1.as_ref().unwrap().next.as_ref().unwrap()
        );

        println!(
            "address of list2's node3 on heap = {:p}",
            &**list2.as_ref().unwrap()
        );
        println!(
            "address of list2's node4 on heap = {:p}",
            &**list2.as_ref().unwrap().next.as_ref().unwrap()
        );

        // cargo test -- --show-output
        let mut result = Solution::merge_two_lists(list1, list2);
        println!("{:?}", result); // 1 -> 1 -> 2 -> 4 node1 -> node3 -> node2 -> node4

        // `to_owned()` will clone the data..
        // so except for node1 and node3 (the first node of the 2 lists),
        // all other nodes' addresses are changed.
        // With better version, addresses of all the nodes are unchanged!

        let mut node = result.take().unwrap();
        println!("address of result's node1 on heap = {:p}", &*node);
        node = node.next.take().unwrap();
        println!("address of result's node3 on heap = {:p}", &*node);
        node = node.next.take().unwrap();
        println!("address of result's node2 on heap = {:p}", &*node);
        node = node.next.take().unwrap();
        println!("address of result's node4 on heap = {:p}", &*node);
    }
}
