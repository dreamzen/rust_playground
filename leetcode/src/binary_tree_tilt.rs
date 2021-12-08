use std::cell::RefCell;
use std::rc::Rc;

use crate::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut tilt = 0;
        Self::dfs(&root, &mut tilt);
        tilt
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, total_tilt: &mut i32) -> i32 {
        match root {
            None => return 0,
            Some(r) => {
                let left_sum = Self::dfs(&r.borrow().left, total_tilt);
                let right_sum = Self::dfs(&r.borrow().right, total_tilt);
                let curr_tilt = (left_sum - right_sum).abs();
                *total_tilt = *total_tilt + curr_tilt;
                return left_sum + right_sum + &r.borrow().val;
            }
        }
    }
}
