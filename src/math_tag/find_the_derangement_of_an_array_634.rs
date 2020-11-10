/*
 * @lc app=leetcode.cn id=634 lang=rust
 *
 * [634] 寻找数组的错位排列
 */

// @lc code=start
const MOD: i64 = (1e9 as i64) + 7;

impl Solution {
    pub fn find_derangement(n: i32) -> i32 {
        let n = n as i64;
        if n <= 1 {
            return 0;
        }
        let mut mul = 1i64;
        let mut sum = 0i64;
        for i in (0..=n).rev() {
            sum = (sum + MOD + mul * (if i % 2 == 0 { 1 } else { -1 })) % MOD;
            mul = (mul * i) % MOD;
        }
        sum as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_derangement_1() {
        assert_eq!(Solution::find_derangement(3), 2);
    }

    #[test]
    fn test_find_derangement_2() {
        assert_eq!(Solution::find_derangement(1), 0);
    }

    
    #[test]
    fn test_find_derangement_3() {
        assert_eq!(Solution::find_derangement(4), 9);
    }
}