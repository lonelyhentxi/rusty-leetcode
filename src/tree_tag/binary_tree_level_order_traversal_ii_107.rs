/*
 * @lc app=leetcode.cn id=107 lang=rust
 *
 * [107] 二叉树的层次遍历 II
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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut qs = [VecDeque::new(),VecDeque::new()];
        let mut res = vec![];
        qs[0].push_back(root);
        let mut current = 0usize;
        loop {
            let next = (current+1)%2;
            let mut current_res = vec![];
            let mut counter = 0;
            while let Some(opt) = qs[current].pop_front() {
                if let Some(node_ref) = opt {
                    let node_borrow = node_ref.borrow();
                    current_res.push(node_borrow.val);
                    qs[next].push_back(node_borrow.left.clone());
                    qs[next].push_back(node_borrow.right.clone());
                    counter += 1;
                }
            }
            if counter == 0 {
                break;
            }
            res.push(current_res);
            current = next;
        }
        res.reverse();
        res
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
    fn test_level_order_bottom() {
        let tree = tree_node!(
            3,
            tree_leaf!(9),
            tree_node!(20, tree_leaf!(15), tree_leaf!(7))
        );
        let expected = vec![
            vec![15,7],
            vec![9,20],
            vec![3]
        ];
        assert_eq!(Solution::level_order_bottom(tree), expected);
    }
}