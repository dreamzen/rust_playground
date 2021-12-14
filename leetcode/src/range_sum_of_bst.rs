use std::cell::RefCell;
use std::rc::Rc;

use crate::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        return Self::dfs(&root, low, high);
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => return 0,
            Some(t) => {
                let val = t.borrow().val;
                if val < low {
                    return Self::dfs(&t.borrow().right, low, high);
                } else if val > high {
                    return Self::dfs(&t.borrow().left, low, high);
                } else {
                    return val
                        + Self::dfs(&t.borrow().left, low, high)
                        + Self::dfs(&t.borrow().right, low, high);
                }
            }
        }
    }
}
