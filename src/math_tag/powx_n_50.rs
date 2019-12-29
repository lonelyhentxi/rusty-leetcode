/*
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * [50] Pow(x, n)
 */

// @lc code=start
impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        let mut n = i64::from(n);
        if n < 0 {
            x = 1f64 / x;
            n = -n;
        }
        let mut factor= 1f64;
        let mut product = x;
        while n > 0 {
            if (n&1)==1 {
                factor *= product;
            }
            product *= product;
            n/=2;
        }
        factor
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::assert_feq;

    #[test]
    fn test_my_pow() {
        assert_feq(Solution::my_pow(2.0f64, 10), 1024f64);
        assert_feq(Solution::my_pow(2.1f64, 3), 9.261f64);
        assert_feq(Solution::my_pow(2f64, -2),0.25f64);
    }
}