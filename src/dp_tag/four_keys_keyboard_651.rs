/*
 * @lc app=leetcode.cn id=651 lang=rust
 *
 * [651] 4键键盘
 */

// @star
// @lc code=start
impl Solution {
    pub fn max_a(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0usize, 1];
        for k in 2..=n {
            let mut m = usize::min_value();
            for x in 0..k - 1 {
                m = usize::max(dp[x] * (k - x - 1), m);
            }
            m = usize::max(dp[dp.len() - 1] + 1, m);
            dp.push(m);
        }
        dp[n] as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_max_a_1() {
        assert_eq!(Solution::max_a(3), 3);
        assert_eq!(Solution::max_a(7), 9);
    }
}