/*
 * @lc app=leetcode.cn id=573 lang=rust
 *
 * [573] 松鼠模拟
 */

// @lc code=start
impl Solution {
    fn distance(a: &[i32], b: &[i32]) -> i32 {
        i32::abs(a[0] - b[0]) + i32::abs(a[1] - b[1])
    }

    pub fn min_distance(_: i32, _: i32, tree: Vec<i32>, squirrel: Vec<i32>, nuts: Vec<Vec<i32>>) -> i32 {
        if nuts.is_empty() {
            return 0;
        }
        let nuts_distance = nuts.iter().fold(0, |acc, curr| {
            acc + Solution::distance(&tree, &curr) * 2
        });
        if tree == squirrel {
            return nuts_distance;
        }
        let min_first_distance = nuts.iter().map(|n| Solution::distance(n, &squirrel)- Solution::distance(n, &tree) ).min().unwrap();
        nuts_distance + min_first_distance
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_min_distance_1() {
        assert_eq!(Solution::min_distance(5, 7, vec![2, 2], vec![4, 4], lc_matrix![[3,0], [2,5]]), 12);
    }
}