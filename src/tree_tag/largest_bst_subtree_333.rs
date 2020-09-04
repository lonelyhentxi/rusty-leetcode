use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=333 lang=rust
 *
 * [333] 最大 BST 子树
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

#[derive(Debug)]
struct LargetBSTSolution {
    max_bst: usize,
    is_bst: bool,
    min: i32,
    max: i32,
}

impl Solution {
    pub fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::largest_bst_subtree_recursive(root).max_bst as i32
    }

    fn largest_bst_subtree_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> LargetBSTSolution {
        match root {
            None => LargetBSTSolution {
                max_bst: 0,
                is_bst: true,
                min: i32::max_value(),
                max: i32::min_value(),
            },
            Some(node_ref) => {
                let mut node = node_ref.borrow_mut();
                let value = node.val;
                let left_solution = Solution::largest_bst_subtree_recursive(node.left.take());
                let right_solution = Solution::largest_bst_subtree_recursive(node.right.take());
                if left_solution.is_bst
                    && right_solution.is_bst
                    && left_solution.max < value
                    && right_solution.min > value
                {
                    LargetBSTSolution {
                        max_bst: left_solution.max_bst + right_solution.max_bst + 1,
                        is_bst: true,
                        min: i32::min(left_solution.min, value),
                        max: i32::max(right_solution.max, value),
                    }
                } else {
                    LargetBSTSolution {
                        max_bst: usize::max(left_solution.max_bst, right_solution.max_bst),
                        is_bst: false,
                        min: i32::min(left_solution.min, value),
                        max: i32::max(right_solution.max, value),
                    }
                }
            }
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
    fn test_largest_bst_subtree_1() {
        let tree = tree_node!(10, 
            tree_node!(5, 
                tree_leaf!(1), 
                tree_leaf!(8)
            ),
            tree_node!(15, None, tree_leaf!(7))
        );
        assert_eq!(Solution::largest_bst_subtree(tree), 3);
    }

    #[test]
    fn test_largest_bst_subtree_2() {
        let tree = tree_node!(4, 
            tree_node!(2, 
                tree_node!(2,
                    tree_node!(2,
                        tree_leaf!(1),
                        None
                    ),
                    None
                ), 
                tree_leaf!(3)
            ),
            tree_node!(7,
                tree_leaf!(5),
                None
            )
        );
        assert_eq!(Solution::largest_bst_subtree(tree), 2);
    }
}