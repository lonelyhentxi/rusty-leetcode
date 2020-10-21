use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=545 lang=rust
 *
 * [545] 二叉树的边界
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

enum BoundaryState {
    Root,
    Left,
    Right,
    Leaf,
    Unknown,
}

impl Solution {
    fn boundary_of_binary_tree_recursive(
        curr: Option<Rc<RefCell<TreeNode>>>,
        mut bs: BoundaryState,
        mut res: &mut Vec<i32>,
    ) {
        if let Some(curr_ref) = curr {
            let mut curr_node = curr_ref.borrow_mut();
            if curr_node.left.is_none() && curr_node.right.is_none() {
                bs = BoundaryState::Leaf;
            }
            match bs {
                BoundaryState::Root => {
                    res.push(curr_node.val);
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.left.take(),
                        BoundaryState::Left,
                        &mut res,
                    );
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.right.take(),
                        BoundaryState::Right,
                        &mut res,
                    );
                }
                BoundaryState::Left => {
                    res.push(curr_node.val);
                    let left_empty = curr_node.left.is_none();
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.left.take(),
                        BoundaryState::Left,
                        &mut res,
                    );
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.right.take(),
                        if left_empty {
                            BoundaryState::Left
                        } else {
                            BoundaryState::Unknown
                        },
                        &mut res,
                    );
                }
                BoundaryState::Right => {
                    let right_empty = curr_node.right.is_none();
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.left.take(),
                        if right_empty { BoundaryState::Right } else { BoundaryState::Unknown },
                        &mut res,
                    );
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.right.take(),
                        BoundaryState::Right,
                        &mut res,
                    );
                    res.push(curr_node.val);
                }
                BoundaryState::Unknown => {
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.left.take(),
                        BoundaryState::Unknown,
                        &mut res,
                    );
                    Solution::boundary_of_binary_tree_recursive(
                        curr_node.right.take(),
                        BoundaryState::Unknown,
                        &mut res,
                    );
                }
                BoundaryState::Leaf => {
                    res.push(curr_node.val);
                }
            }
        }
    }

    pub fn boundary_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Solution::boundary_of_binary_tree_recursive(root, BoundaryState::Root, &mut res);
        res
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tree_leaf, tree_node};

    #[test]
    fn test_boundary_of_binary_tree_1() {
        let tree = tree_node!(1, None, tree_node!(2, tree_leaf!(3), tree_leaf!(4)));
        assert_eq!(Solution::boundary_of_binary_tree(tree), vec![1, 3, 4, 2]);
    }

    #[test]
    fn test_boundary_of_binary_tree_2() {
        let tree = tree_node!(
            1,
            tree_node!(
                2,
                tree_leaf!(4),
                tree_node!(5, tree_leaf!(7), tree_leaf!(8))
            ),
            tree_node!(3, tree_node!(6, tree_leaf!(9), tree_leaf!(10)), None)
        );
        assert_eq!(
            Solution::boundary_of_binary_tree(tree),
            vec![1, 2, 4, 7, 8, 9, 10, 6, 3]
        );
    }
}
