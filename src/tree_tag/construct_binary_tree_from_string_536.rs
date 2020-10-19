use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=536 lang=rust
 *
 * [536] 从字符串生成二叉树
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
    pub fn str2tree(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let chars = {
            let mut cs = vec!['('];
            cs.extend(s.chars());
            cs.push(')');
            cs
        };
        let mut working = vec![];
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            if ch == '(' {
                let mut curr = VecDeque::new();
                let mut j = i + 1;
                loop {
                    let ch = chars[j];
                    if ch == ')' || ch == '(' {
                        break;
                    }
                    j += 1;
                }
                curr.push_back(Solution::to_num(&chars[(i + 1)..j]));
                working.push(curr);
                i = j;
            } else if ch == ')' {
                let mut curr = working.pop().unwrap();
                let new_node = curr.pop_front().unwrap().map(|p_ref| {
                    {
                        let mut p_node = p_ref.borrow_mut();
                        p_node.left = curr.pop_front().unwrap_or(None);
                        p_node.right = curr.pop_front().unwrap_or(None);
                    }
                    p_ref
                });
                let len = working.len();
                if working.len() == 0 {
                    working.push(VecDeque::new());
                    working[len].push_back(new_node);
                } else {
                    working[len - 1].push_back(new_node);
                }
                i += 1;
            }
        }
        working[0].pop_front().unwrap()
    }

    fn to_num(s: &[char]) -> Option<Rc<RefCell<TreeNode>>> {
        s.iter().cloned().collect::<String>().parse::<i32>().map(|i| Rc::new(RefCell::new(TreeNode::new(i)))).ok()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ tree_node,tree_leaf };

    #[test]
    fn test_str2tree_1() {
        let tree = tree_node!(4, 
            tree_node!(2,
                tree_leaf!(3),
                tree_leaf!(1)
            ),
            tree_node!(6,
                tree_leaf!(5),
                None
            )
        );
        assert_eq!(Solution::str2tree(String::from("4(2(3)(1))(6(5))")), tree);
    }
}