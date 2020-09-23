use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=366 lang=rust
 *
 * [366] 寻找二叉树的叶子节点
 */

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn find_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
      let mut res: Vec<Vec<i32>> = vec![];
      Solution::find_leaves_recursive(root, &mut res);
      res
    }

    fn find_leaves_recursive(root: Option<Rc<RefCell<TreeNode>>>, mut res: &mut Vec<Vec<i32>>) -> i32 {
      match root {
        Some(node_ref) => {
          let mut node = node_ref.borrow_mut();
          let val = node.val;
          let left_leaf_distance = Solution::find_leaves_recursive(node.left.take(), &mut res);
          let right_leaf_distance = Solution::find_leaves_recursive(node.right.take(), &mut res);
          let current_leaf_distance = i32::max(left_leaf_distance, right_leaf_distance) + 1;
          if (res.len() as i32) <= current_leaf_distance  - 1 {
            res.push(vec![val]);
          } else {
            res[(current_leaf_distance  - 1) as usize].push(val);
          }
          current_leaf_distance
        },
        None => 0
      }
    }
}

// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tree_node, tree_leaf};

    #[test]
    fn test_find_leaves() {
      let tree = tree_node!(
        1,
        tree_node!(2,
          tree_leaf!(4),
          tree_leaf!(5)
        ),
        tree_leaf!(3)
      );

      assert_eq!(Solution::find_leaves(tree), vec![vec![4,5,3], vec![2], vec![1]]);
    }
}