/*
 * @lc app=leetcode.cn id=572 lang=rust
 *
 * [572] 另一个树的子树
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
impl Solution {
    pub fn is_subtree(s: Option<Rc<RefCell<TreeNode>>>, t: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let s_actions = Rc::new(RefCell::new(vec![]));
        Solution::log_traverse_actions(&s, s_actions.clone()); // O(m)
        let t_actions = Rc::new(RefCell::new(vec![]));
        Solution::log_traverse_actions(&t, t_actions.clone()); // O(n)
        let s_actions = s_actions.borrow().join(",");
        let t_actions = t_actions.borrow().join(",");
        s_actions.contains(&t_actions) // kmp O(m + n)
    }

    fn log_traverse_actions(r: &Option<Rc<RefCell<TreeNode>>>,actions: Rc<RefCell<Vec<String>>>) {
        match r {
            None => actions.borrow_mut().push("n".to_string()),
            Some(node_ref) => {
                let node_borrow = node_ref.borrow();
                {
                    let mut actions_mut = actions.borrow_mut();
                    actions_mut.push("a".to_string());
                }
                Solution::log_traverse_actions(&node_borrow.left, actions.clone());
                {
                    let mut actions_mut = actions.borrow_mut();
                    actions_mut.push("b".to_string());
                    actions_mut.push(node_borrow.val.to_string());
                    actions_mut.push("c".to_string());
                }
                Solution::log_traverse_actions(&node_borrow.right, actions.clone());
                {
                    let mut actions_mut = actions.borrow_mut();
                    actions_mut.push("c".to_string());
                }
            }
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
    fn test_is_subtree1() {
        let tree1 = tree_node!(
            3,
            tree_node!(4, tree_leaf!(1), tree_leaf!(2)),
            tree_leaf!(5)
        );
        let tree2 = tree_node!(
            4,tree_leaf!(1), tree_leaf!(2)
        );
        assert!(Solution::is_subtree(tree1, tree2));
    }

    
    #[test]
    fn test_is_subtree2() {
        let tree1 = tree_node!(
            3,
            tree_node!(4, tree_leaf!(1), tree_node!(2, tree_leaf!(0), None)),
            tree_leaf!(5)
        );
        let tree2 = tree_node!(
            4,tree_leaf!(1), tree_leaf!(2)
        );
        assert!(!Solution::is_subtree(tree1, tree2));
    }
}