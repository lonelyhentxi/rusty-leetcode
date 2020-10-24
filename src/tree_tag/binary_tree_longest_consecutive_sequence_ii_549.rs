use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=549 lang=rust
 *
 * [549] 二叉树中最长的连续序列
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

struct Data {
    val: i32,
    increase: Option<usize>,
    decease: Option<usize>,
}

impl Solution {
    pub fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0usize;
        Solution::longest_consecutive_recursive(root, &mut max);
        max as i32
    }

    fn longest_consecutive_recursive(
        curr: Option<Rc<RefCell<TreeNode>>>,
        mut max_len: &mut usize,
    ) -> Data {
        match curr {
            Some(curr_ref) => {
                let mut curr_node = curr_ref.borrow_mut();
                let val = curr_node.val;
                let left_res =
                    Solution::longest_consecutive_recursive(curr_node.left.take(), &mut max_len);
                let right_res =
                    Solution::longest_consecutive_recursive(curr_node.right.take(), &mut max_len);
                let increase_left = if left_res.val + 1 == val {
                    left_res.increase.unwrap_or(0)
                } else {
                    0
                };
                let decrease_left = if left_res.val == 1 + val {
                    left_res.decease.unwrap_or(0)
                } else {
                    0
                };
                let increase_right = if right_res.val + 1 == val {
                    right_res.increase.unwrap_or(0)
                } else {
                    0
                };
                let decrease_right = if right_res.val == 1 + val {
                    right_res.decease.unwrap_or(0)
                } else {
                    0
                };
                *max_len = usize::max(*max_len, usize::max(increase_left + decrease_right, decrease_left + increase_right) + 1);
                Data {
                    val,
                    increase: Some(usize::max(increase_left, increase_right) + 1),
                    decease: Some(usize::max(decrease_left, decrease_right) + 1),
                }
            }
            None => Data {
                val: 0,
                increase: None,
                decease: None,
            },
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ tree_node,tree_leaf };

    #[test]
    fn test_longest_consecutive_1() {
        let tree = tree_node!(1, tree_leaf!(2), tree_leaf!(3));
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }

    #[test]
    fn test_longest_consecutive_2() {
        let tree = tree_node!(2, tree_leaf!(1), tree_leaf!(3));
        assert_eq!(Solution::longest_consecutive(tree), 3);
    }

    #[test]
    fn test_longest_consecutive_3() {
        let tree = tree_node!(1, tree_node!(2, tree_leaf!(4), None), tree_leaf!(3));
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }

    #[test]
    fn test_longest_consecutive_4() {
        let tree = tree_node!(3, tree_leaf!(1), tree_leaf!(2));
        assert_eq!(Solution::longest_consecutive(tree), 2);
    }
}
