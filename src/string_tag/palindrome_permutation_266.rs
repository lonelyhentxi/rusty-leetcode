/*
 * @lc app=leetcode.cn id=266 lang=rust
 *
 * [266] 回文排列
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut dict = HashMap::<char, usize>::new();
        for c in s.chars() {
            dict.entry(c)
                .and_modify(|c| *c+=1)
                .or_insert(1);
        }
        let mut can_odd = (s.len() & 1)==1;
        for v in dict.values() {
            if ((*v)&1)==1 {
                if can_odd {
                    can_odd = false;
                } else {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_permute_palindrome() {
        assert!(!Solution::can_permute_palindrome(String::from("code")));
        assert!(Solution::can_permute_palindrome(String::from("aab")));
        assert!(Solution::can_permute_palindrome(String::from("carerac")));
    }
}