/*
 * @lc app=leetcode.cn id=660 lang=rust
 *
 * [660] 移除 9
 */

// @lc code=start
impl Solution {
    pub fn new_integer(mut n: i32) -> i32 {
        let mut res = 0;
        let mut times = 0;
        while n > 0 {
            res += (n % 9) * i32::pow(10, times);
            n /= 9;
            times += 1;
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
    fn test_new_integer_1() {
        assert_eq!(Solution::new_integer(9), 10);
    }

    #[test]
    fn test_new_integer_2() {
        assert_eq!(Solution::new_integer(82), 101);
    }
}