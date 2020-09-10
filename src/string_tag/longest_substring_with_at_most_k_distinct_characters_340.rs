/*
 * @lc app=leetcode.cn id=340 lang=rust
 *
 * [340] 至多包含 K 个不同字符的最长子串
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        if k <= 0 {
            return k;
        }
        let chars = s.chars().collect::<Vec<char>>();
        if chars.len() <= 1 {
            return chars.len() as i32;
        }
        let mut unique = HashMap::<char, usize>::new();
        let mut max_len = 0usize;
        let mut left = -1i32;
        for right in 0..chars.len() {
            let ch = chars[right];
            unique.entry(ch).and_modify(|c| *c+= 1).or_insert(1);
            while unique.len() > k as usize {
                left += 1;
                let lch = chars[left as usize];
                {
                    let count = unique[&lch];
                    if count == 1 {
                        unique.remove(&lch);
                    } else {
                        unique.insert(lch, count - 1);
                    }
                }
            }
            max_len = usize::max(max_len, (right as i32 - left) as usize);
        }
        max_len as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_length_of_longest_substring_k_distinct_1() {
        assert_eq!(Solution::length_of_longest_substring_k_distinct(String::from("eceba"), 2), 3);
    }

    #[test]
    fn test_length_of_longest_substring_k_distinct_2() {
        assert_eq!(Solution::length_of_longest_substring_k_distinct(String::from("aa"), 1), 2);
    }
}