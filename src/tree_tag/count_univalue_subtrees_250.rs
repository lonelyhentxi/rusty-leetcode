/*
 * @lc app=leetcode.cn id=250 lang=rust
 *
 * [250] 统计同值子树
 */
use crate::utils::tree::TreeNode;
// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;

enum TreeState {
    Uni((usize, i32)),
    Diff(usize),
    Empty
}

impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use TreeState::{Diff,Uni,Empty};
        let res = Solution::count_unival_subtrees_rec(&root);
        (match res {
            Diff(count) => count,
            Uni((count, _)) => count,
            Empty => 0
        }) as i32
    }

    fn count_unival_subtrees_rec(root: &Option<Rc<RefCell<TreeNode>>>) -> TreeState {
        use TreeState::{Diff,Uni,Empty};
        match root {
            Some(node_ref) => {
                let node_b = node_ref.borrow();
                let left = Solution::count_unival_subtrees_rec(&node_b.left);
                let right = Solution::count_unival_subtrees_rec(&node_b.right);
                let mut is_uni = true;
                let mut uni_count = 0;
                match left {
                    Uni((count, value)) => {
                        uni_count += count;
                        if value!=node_b.val {
                            is_uni = false;
                        }
                    },
                    Diff(count) => {
                        uni_count += count;
                        is_uni = false;
                    },
                    Empty => {}
                }
                match right {
                    Uni((count, value)) => {
                        uni_count += count;
                        if value!=node_b.val {
                            is_uni = false;
                        }
                    },
                    Diff(count) => {
                        uni_count += count;
                        is_uni = false;
                    },
                    Empty => {}
                }
                if is_uni {
                    uni_count += 1;
                    Uni((uni_count, node_b.val))
                } else {
                    Diff(uni_count)
                }
            },
            None => {
                Empty
            }
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
    fn test_count_unival_subtrees() {
        let tree = tree_node!(5,
            tree_node!(1,
                tree_leaf!(5),
                tree_leaf!(5)
                ),
            tree_node!(5, None, tree_leaf!(5))
            );
        assert_eq!(Solution::count_unival_subtrees(tree), 4);
    }
}