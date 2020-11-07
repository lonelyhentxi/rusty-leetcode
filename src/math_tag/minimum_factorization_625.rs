/*
 * @lc app=leetcode.cn id=625 lang=rust
 *
 * [625] 最小因式分解
 */

// @lc code=start
impl Solution {
    pub fn smallest_factorization(a: i32) -> i32 {
        if a < 2 {
            return a;
        }
        let mut a = a as i64;
        let mut res = 0i64;
        let mut mul = 1i64;
        for i in (2..=9).rev() {
            while a % i == 0 {
                a /= i;
                res = i * mul + res;
                mul *= 10;
            }
        }
        if a < 2 && res <= (i32::max_value() as i64) { res as i32 } else { 0 } 
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_smallest_factorization_1() {
        assert_eq!(Solution::smallest_factorization(48), 68);
    }

    #[test]
    fn test_smallest_factorization_2() {
        assert_eq!(Solution::smallest_factorization(15), 35);
    }

    #[test]
    fn test_smallest_factorization_3() {
        assert_eq!(Solution::smallest_factorization(i32::max_value()), 0);
    }
}