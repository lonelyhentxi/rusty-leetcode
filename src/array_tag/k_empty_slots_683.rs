/*
 * @lc app=leetcode.cn id=683 lang=rust
 *
 * [683] K 个关闭的灯泡
 */

// @lc code=start
use std::collections::BTreeSet;
use std::ops::Bound::{ Excluded, Unbounded };

impl Solution {
    pub fn k_empty_slots(bulbs: Vec<i32>, k: i32) -> i32 {
        let mut opened = BTreeSet::<i32>::new();
        for i in 0..bulbs.len() {
            let index = bulbs[i] - 1;
            let day = i as i32 + 1;
            opened.insert(index);
            let before = opened.range((Unbounded, Excluded(&index))).rev().next();
            let after = opened.range((Excluded(&index), Unbounded)).next();
            if let Some(&b) = before {
                if i32::abs(index - b) == k + 1 {
                    return day;
                }
            }
            if let Some(&a) = after {
                if i32::abs(index - a) == k + 1 {
                    return day;
                }
            }
        }
        -1
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_k_empty_slots_1() {
        assert_eq!(Solution::k_empty_slots(vec![1,3,2], 1), 2);
    }

    #[test]
    fn test_k_empty_slots_2() {
        assert_eq!(Solution::k_empty_slots(vec![1,2,3], 1), -1);
    }
}