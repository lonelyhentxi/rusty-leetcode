/*
 * @lc app=leetcode.cn id=548 lang=rust
 *
 * [548] 将数组分割成和相等的子数组
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> bool {
        let len = nums.len();
        if len < 7 {
            return false;
        }
        let mut sum = 0;
        let mut sum_dp = vec![0; len];
        let mut dict = HashMap::<i32, Vec<usize>>::new();
        for i in 0..len {
            let n = nums[i];
            sum_dp[i] = sum;
            sum += n;
            dict
                .entry(n)
                .and_modify(|v| v.push(i))
                .or_insert_with(|| vec![i]);
        }
        let mut i = 1;
        while i + 6 <= len {
            let mut k = len - 2;
            while i + 4 <= k {
                let left_sum = sum_dp[i];
                let right_sum = sum - sum_dp[k] - nums[k];
                let mid_sum = sum_dp[k] - sum_dp[i] - nums[i];
                if left_sum == right_sum {
                    let n_j =  mid_sum - left_sum - right_sum;
                    if let Some(js) = dict.get(&n_j) {
                        for &j in js {
                            if i + 1 < j && j + 1 < k {
                                return true;
                            } 
                        }
                    }
                }
                k -= 1;
            }
            i += 1;
        }
        false
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_array_1() {
        assert_eq!(Solution::split_array(vec![1,2,1,2,1,2,1]), true);
    }

    
    #[test]
    fn test_split_array_2() {
        assert_eq!(Solution::split_array(vec![-1, 0, -1, 0, -1, 0, -1]), true);
    }
}