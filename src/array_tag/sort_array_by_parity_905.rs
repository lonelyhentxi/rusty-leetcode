/*
 * @lc app=leetcode.cn id=905 lang=rust
 *
 * [905] 按奇偶排序数组
 */

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        if a.len() <= 1 {
            return a;
        }
        let mut res = vec![0;a.len()];
        let mut even_idx = 0usize;
        let mut odd_idx = a.len() - 1;
        for elem in a {
            if elem & 1 == 1 {
                res[odd_idx] = elem;
                if odd_idx > 0 {
                    odd_idx -=1;
                }
            }
            else {
                res[even_idx] = elem;
                even_idx += 1;
            }
        }
        res
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(Solution::sort_array_by_parity(vec![3,1,2,4]),vec![2,4,1,3]);
        assert_eq!(Solution::sort_array_by_parity(vec![]),vec![]);
    }
}