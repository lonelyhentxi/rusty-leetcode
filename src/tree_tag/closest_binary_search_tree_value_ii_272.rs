/*
 * @lc app=leetcode.cn id=272 lang=rust
 *
 * [272]  最接近的二叉搜索树值 II
 */
use crate::utils::tree::TreeNode;
// @lc code=start
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

enum NodeOrVal {
    Node(Option<Rc<RefCell<TreeNode>>>),
    Val(i32),
}

#[derive(PartialEq, PartialOrd)]
struct NonNan(f64);

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn closest_k_values(root: Option<Rc<RefCell<TreeNode>>>, target: f64, k: i32) -> Vec<i32> {
        let sorted = Solution::inorder_traversal(root);
        Solution::find_closest(&sorted, target, k as isize)
    }

    pub fn find_closest(arr: &[i32], target: f64, k: isize) -> Vec<i32> {
        if (arr.len() as isize) <= k {
            return arr.iter().cloned().collect();
        }
        let mut begin = 0isize;
        let mut end = arr.len() as isize;
        let mut heap = BinaryHeap::<(NonNan, i32)>::new();
        while begin + 1 < end {
            let mid = (begin + end) / 2;
            let mid_val = f64::from(arr[mid as usize] as i32);
            if mid_val <= target {
                begin = mid;
            } else {
                end = mid;
            }
        }
        let mut i = begin;
        while i >= 0 && i >= begin - k {
            let sub 
                = (NonNan(f64::abs(arr[i as usize] as f64 - target)), arr[i as usize]);
            Solution::cmp_add(&mut heap, sub, k);
            i-=1;
        }
        i = begin + 1;
        while i < (arr.len() as isize) && i <= begin + k {
            let sub 
                = (NonNan(f64::abs(arr[i as usize] as f64 - target)), arr[i as usize]);
            Solution::cmp_add(&mut heap, sub, k);
            i+=1;
        }
        heap.iter().map(|(_, v)| *v).collect()
    }

    pub fn cmp_add(heap: &mut BinaryHeap<(NonNan, i32)>, elem: (NonNan, i32), k: isize) {
        if heap.len() < k as usize {
            heap.push(elem);
        } else {
            let top = heap.pop().unwrap();
            if top.0 < elem.0 {
                heap.push(top);
            } else {
                heap.push(elem);
            }
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use NodeOrVal::{Node, Val};
        let mut values = vec![];
        let mut stack: Vec<NodeOrVal> = vec![Node(root)];
        while let Some(node_or_val) = stack.pop() {
            match node_or_val {
                Node(node) => {
                    if let Some(node_ref) = node {
                        let node_b = node_ref.borrow();
                        stack.push(Node(node_b.right.clone()));
                        stack.push(Val(node_b.val));
                        stack.push(Node(node_b.left.clone()));
                    }
                }
                Val(val) => {
                    values.push(val);
                }
            }
        }
        values
    }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::assert_equivalent;
    use crate::{tree_leaf, tree_node};

    #[test]
    fn test_closest_k_values() {
        let tree = tree_node!(
            4,
            tree_node!(2, tree_leaf!(1), tree_leaf!(3)),
            tree_leaf!(5)
        );
        assert_equivalent(&Solution::closest_k_values(tree, 3.714286, 2), &[3, 4]);
    }

    #[test]
    fn test_closest_k_values1() {
        let tree = tree_node!(
            8,
            tree_leaf!(1),
            None
        );
        assert_equivalent(&Solution::closest_k_values(tree, 6., 1), &[8]);
    }
}
