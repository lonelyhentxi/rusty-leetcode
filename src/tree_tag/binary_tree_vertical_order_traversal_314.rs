use crate::utils::tree::TreeNode;
/*
 * @lc app=leetcode.cn id=314 lang=rust
 *
 * [314] 二叉树的垂直遍历
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
use std::collections::{ VecDeque, hash_map::HashMap };
use std::rc::Rc;

impl Solution {
    pub fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, root));
        while let Some((level, node_op)) = queue.pop_front() {
            if let Some(node_ref) = node_op {
                let mut node = node_ref.borrow_mut();
                let value = node.val;
                levels
                    .entry(level)
                    .and_modify(|arr| {
                        arr.push(value);
                    })
                    .or_insert(vec![value]);
                queue.push_back((level - 1, node.left.take()));
                queue.push_back((level + 1, node.right.take()));
            }
        }
        let mut ret = levels
            .into_iter()
            .collect::<Vec<(i32, Vec<i32>)>>();
        ret.sort_by_key(|(k,_)| *k);
        ret.into_iter().map(|(_, v)| v).collect()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{tree_leaf, tree_node};

    #[test]
    fn test_binary_tree_vertical_order_traversal_1() {
        let tree = tree_node!(
            3,
            tree_leaf!(9),
            tree_node!(20, tree_leaf!(15), tree_leaf!(7))
        );
        assert_eq!(
            Solution::vertical_order(tree),
            vec![vec![9], vec![3, 15], vec![20], vec![7]]
        );
    }

    #[test]
    fn test_binary_tree_vertical_order_traversal_2() {
        let tree = tree_node!(
            3,
            tree_node!(9, tree_leaf!(4), tree_leaf!(0)),
            tree_node!(8, tree_leaf!(1), tree_leaf!(7))
        );
        assert_eq!(
            Solution::vertical_order(tree),
            vec![vec![4], vec![9], vec![3, 0, 1], vec![8], vec![7]]
        );
    }

    #[test]
    fn test_binary_tree_vertical_order_traversal_3() {
        let tree = tree_node!(
            3,
            tree_node!(9, tree_leaf!(4), tree_node!(0, None, tree_leaf!(2))),
            tree_node!(8, tree_node!(1, tree_leaf!(5), None), tree_leaf!(7))
        );
        assert_eq!(
            Solution::vertical_order(tree),
            vec![vec![4], vec![9, 5], vec![3, 0, 1], vec![8, 2], vec![7]]
        );
    }

    
    #[test]
    fn test_binary_tree_vertical_order_traversal_4() {
        let tree = None;
        assert_eq!(
            Solution::vertical_order(tree),
            Vec::<Vec<i32>>::new()
        );
    }
}
