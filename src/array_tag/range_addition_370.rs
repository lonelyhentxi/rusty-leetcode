/*
 * @lc app=leetcode.cn id=370 lang=rust
 *
 * [370] 区间加法
 */

// @lc code=start
impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![0; length as usize];
        for u in updates {
            res[u[0] as usize] += u[2];
            if res.len() > u[1] as usize + 1 {
                res[u[1] as usize + 1] += 0 - u[2];
            }
        }
        res.into_iter().scan(0, |acc, curr| { *acc+=curr; Some(*acc) }).collect()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_addition() {
        assert_eq!(
            Solution::get_modified_array(5, vec![vec![1, 3, 2], vec![2, 4, 3], vec![0, 2, -2]]),
            vec![-2, 0, 3, 5, 3]
        );
    }
}
