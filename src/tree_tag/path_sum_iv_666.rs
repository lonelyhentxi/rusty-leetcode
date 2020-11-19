/*
 * @lc app=leetcode.cn id=666 lang=rust
 *
 * [666] 路径总和 IV
 */

// @lc code=start
use std::collections::{ HashMap, HashSet };

impl Solution {
    pub fn path_sum(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut tree = HashMap::<(i32, i32), i32>::new();
        let mut parents = HashSet::<(i32, i32)>::new();
        tree.insert((-1, 0), 0);
        for n in nums {
            let num = n - 110;
            let layer = num / 100;
            let index = ( num - layer * 100 ) / 10;
            let value = num - layer * 100 - index * 10;
            let parent = (layer - 1, index / 2);
            let parent_value = tree[&parent];
            parents.insert(parent);
            tree.insert((layer, index), parent_value + value);
        }
        tree
            .into_iter()
            .filter(|(k, _)| !parents.contains(k))
            .map(|(_,v)| v)
            .fold(0, |acc, curr| acc + curr)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_sum_1() {
        assert_eq!(Solution::path_sum(vec![113, 215, 221]), 12);
    }

    #[test]
    fn test_path_sum_2() {
        assert_eq!(Solution::path_sum(vec![113, 221]), 4);
    }

    #[test]
    fn test_path_sum_3() {
        assert_eq!(Solution::path_sum(vec![111,217,221,315,415]), 20);
    }
}