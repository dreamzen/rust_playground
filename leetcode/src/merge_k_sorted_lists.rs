use crate::list_node::ListNode;

struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        let mut lists = lists;
        let mut index_of_min = -1;
        let mut min = i32::MAX;

        // find the target list
        for (i, node) in lists.iter().enumerate() {
            if node.is_some() {
                if node.as_ref().unwrap().val < min {
                    index_of_min = i as i32;
                    min = node.as_ref().unwrap().val;
                }
            }
        }

        // merge is complete
        if index_of_min == -1 {
            return None;
        }

        // take the node
        let mut node = lists[index_of_min as usize].take().unwrap();
        lists[index_of_min as usize] = node.next;
        node.next = Solution::merge_k_lists(lists);

        Some(node)
    }
}
