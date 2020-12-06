use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=742 lang=rust
 *
 * [742] 二叉树最近的叶节点
 */

// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{ HashMap, HashSet, VecDeque };

impl Solution {
    pub fn find_closest_leaf(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let (graph, leaves) = Solution::tree_to_graph(root);
        let mut deque = VecDeque::new();
        deque.push_back(k);
        let mut searched = HashSet::new();
        while let Some(v) = deque.pop_front() {
            if leaves.contains(&v) {
                return v;
            }
            if searched.contains(&v) {
                continue;
            }
            searched.insert(v);
            for &c in &graph[&v] {
                deque.push_back(c);
            }
        }
        unreachable!()
    }

    fn tree_to_graph(root: Option<Rc<RefCell<TreeNode>>>) -> ( HashMap<i32, Vec<i32>>, HashSet<i32>) {
        let mut graph = HashMap::<i32, Vec<i32>>::new();
        let mut queue: Vec<(Option<Rc<RefCell<TreeNode>>>, Option<i32>)> = vec![(root, None)];
        let mut counts = HashMap::new();
        while let Some((n_opt,  p_opt)) = queue.pop() {
            if let Some(n_ref) = n_opt {
                let n_node = n_ref.borrow();
                let n_val = n_node.val;
                counts.entry(n_val).or_insert(0);
                if let Some(p_val) = p_opt {
                    graph.entry(n_val).and_modify(|v| v.push(p_val)).or_insert_with(|| vec![p_val]);
                    graph.entry(p_val).and_modify(|v| v.push(n_val)).or_insert_with(|| vec![n_val]);
                    counts.entry(p_val).and_modify(|v| *v += 1);
                };
                queue.push((n_node.left.clone(), Some(n_val)));
                queue.push((n_node.right.clone(), Some(n_val)));   
            }
        }
        (graph, counts.into_iter().filter(|(_, v)| *v == 0).map(|(k, _)| k).collect())
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lc_tree;

    #[test]
    fn test_find_closest_leaf_1() {
        let tree = lc_tree![1, 3, 2];
        let maybe = (vec![2, 3]).into_iter().collect::<HashSet<i32>>();
        assert!(maybe.contains(&Solution::find_closest_leaf(tree, 1)));
    }

    #[test]
    fn test_find_closest_leaf_2() {
        let tree = lc_tree![1];
        let maybe = (vec![1]).into_iter().collect::<HashSet<i32>>();
        assert!(maybe.contains(&Solution::find_closest_leaf(tree, 1)));
    }

    #[test]
    fn test_find_closest_leaf_3() {
        let tree = lc_tree![1,2,3,4,null,null,null,5,null,6];
        let maybe = (vec![3]).into_iter().collect::<HashSet<i32>>();
        assert!(maybe.contains(&Solution::find_closest_leaf(tree, 2)));
    }
}