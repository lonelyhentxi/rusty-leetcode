/*
 * @lc app=leetcode.cn id=270 lang=rust
 *
 * [270]  最接近的二叉搜索树值
 */
use crate::utils::tree::TreeNode;
// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        Solution::closest_value_rec(&root, target).unwrap()
    }

    fn closest_value_rec(root: &Option<Rc<RefCell<TreeNode>>>, target: f64) -> Option<i32> {
        match root {
            Some(node_ref) => {
                let node_b = node_ref.borrow();
                let mut another_val: i32 = i32::min_value();
                let mut has_another = false;
                if target < (node_b.val as f64) {
                    if let Some(left_max) = Solution::closest_value_rec(&node_b.left, target) {
                        another_val = left_max;
                        has_another = true;
                    }
                } else if target > (node_b.val as f64) {
                    if let Some(right_min) = Solution::closest_value_rec(&node_b.right, target) {
                        another_val = right_min;
                        has_another = true;
                    }
                }
                if has_another {
                    if f64::abs((another_val as f64) - target) < f64::abs((node_b.val as f64) - target) {
                        Some(another_val)
                    } else {
                        Some(node_b.val)
                    }
                } else {
                    Some(node_b.val)
                }
            },
            None => None
        }
    }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tree_node,tree_leaf};

    #[test]
    fn test_closest_value_rec() {
        let src = tree_node!(4,
            tree_node!(2, tree_leaf!(1), tree_leaf!(3)),
            tree_leaf!(5));
        assert_eq!(Solution::closest_value(src, 3.714286), 4);
    }
}