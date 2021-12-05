use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::tree_node::TreeNode;

struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (money_with_root, money_without_root) = Self::dfs(&root);
        money_with_root.max(money_without_root)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match root {
            None => return (0, 0),
            Some(r) => {
                let (l1, l2) = Self::dfs(&r.borrow().left);
                let (r1, r2) = Self::dfs(&r.borrow().right);
                (
                    // money with root: val + money without left + money without right
                    r.borrow().val + l2 + r2,
                    // money without root: max(money with left, money without left) + max (right)
                    l1.max(l2) + r1.max(r2),
                )
            }
        }
    }
}
