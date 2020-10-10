/*
 * @lc app=leetcode.cn id=487 lang=rust
 *
 * [487] 最大连续1的个数 II
 */

// @lc code=start
impl Solution {
    pub fn find_max_consecutive_ones(mut nums: Vec<i32>) -> i32 {
        nums.push(0);
        let mut last_max_len = -1;
        let mut curr_max_len = 0;
        let mut sum_max_len = 0;
        for n in nums {
            match n {
                1 => { 
                    curr_max_len += 1;
                    sum_max_len = i32::max(sum_max_len, last_max_len + curr_max_len + 1);
                },
                0 => {
                    sum_max_len = i32::max(sum_max_len, last_max_len + curr_max_len + 1);
                    last_max_len = curr_max_len;
                    curr_max_len = 0;
                },
                _ => unreachable!()
            }
        }
        sum_max_len
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_find_max_consecutive_ones_1() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0]), 4);
    }

    #[test]
    fn test_find_max_consecutive_ones_2() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
    }
}