/*
 * @lc app=leetcode.cn id=298 lang=rust
 *
 * [298] 二叉树最长连续序列
 */
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::longest_consecutive_rec(&root).2 as i32
    }

    fn longest_consecutive_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> (usize, i32, usize) {
        match root {
            Some(node_ref) => {
                let node_borrow = node_ref.borrow();
                let mut res = (1, node_borrow.val, 1);
                let left_res = Solution::longest_consecutive_rec(&node_borrow.left);
                let right_res = Solution::longest_consecutive_rec(&node_borrow.right);
                if left_res.0!=0 || left_res.1!=0 {
                    if node_borrow.val + 1 == left_res.1 {
                        res.0 = usize::max(1 + left_res.0, res.0);
                    }
                }
                if right_res.0!=0 || right_res.1!=0 {
                    if node_borrow.val + 1 == right_res.1 {
                        res.0 = usize::max(1 + right_res.0, res.0);
                    }
                }
                res.2 = usize::max(
                    usize::max(res.2, res.0), 
                    usize::max(left_res.2, right_res.2)
                );
                res
            },
            None => (0, 0, 0)
        }
    }
}
// @lc code=end

use crate::utils::tree::TreeNode;

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tree_node,tree_leaf};

    #[test]
    fn test_longest_consecutive1() {
        let tree = 
        tree_node!(
            1,
            None,
            tree_node!(
                3,
                tree_leaf!(3),
                tree_node!(
                    4,
                    None,
                    tree_leaf!(5)
                )
            )
        );
        assert_eq!(Solution::longest_consecutive(tree), 3);
    }

    #[test]
    fn test_longest_consecutive2() {
        let tree = 
        tree_node!(
            2,
            None,
            tree_node!(
                3,
                tree_node!(
                    2,
                    tree_leaf!(1),
                    None
                ),
                None
            )
        );
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }
}