use crate::utils::tree::TreeNode;

struct Solution;

/*
 * @lc app=leetcode.cn id=156 lang=rust
 *
 * [156] 上下翻转二叉树
 */

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn right_sibling(
        root: Option<Rc<RefCell<TreeNode>>>, 
        parent: Option<Rc<RefCell<TreeNode>>>, 
        right: Option<Rc<RefCell<TreeNode>>>) 
        -> Option<Rc<RefCell<TreeNode>>> {
            match &root {
                Some(root_ref) => {
                    let res = {
                        let (root_right, root_left) = {
                            let mut root_mb = root_ref.borrow_mut();
                            let root_right = root_mb.right.take();
                            let root_left = root_mb.left.take();
                            Solution::right_sibling(root_right.clone(), None, None);
                            (root_right, root_left)
                        };
                        if root_left.is_some() {
                            Solution::right_sibling(root_left, root.clone(), root_right)
                        } else {
                            root.clone()
                        }
                    };
                    {
                        let mut left_mut = root_ref.borrow_mut();
                        left_mut.left = right;
                        left_mut.right = parent;
                    }
                    res
                },
                None => {
                    None
                }
            }
    }

    pub fn upside_down_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) 
    -> Option<Rc<RefCell<TreeNode>>> {
        Solution::right_sibling(root, None, None)
    }
}


// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{tree_node,tree_leaf};
    #[test]
    fn test_binary_tree_upside_down_basic() {
        let src = tree_node!(1, tree_node!(2, tree_leaf!(4), tree_leaf!(5)), tree_leaf!(3));
        let tar = tree_node!(4, tree_leaf!(5), tree_node!(2, tree_leaf!(3), tree_leaf!(1)));
        assert_eq!(Solution::upside_down_binary_tree(src), tar);
    }
}