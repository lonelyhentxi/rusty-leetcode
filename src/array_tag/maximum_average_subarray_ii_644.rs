/*
 * @lc app=leetcode.cn id=644 lang=rust
 *
 * [644] 子数组最大平均数 II
 */


// @star
// @lc code=start
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let len = nums.len();
        let k = k as usize;
        if len == 0 || k > len || k == 0 {
            return 0f64;
        }
        let mut min_val = nums.iter().cloned().min().unwrap() as f64;
        let mut max_val = nums.iter().cloned().max().unwrap() as f64;
        let mut prev_mid = max_val;
        let mut error = i32::max_value() as f64;
        while error > 0.00001 {
            let mid = (min_val + max_val) / 2.0;
            if Solution::has_k_more_average(&nums, mid, k) {
                min_val = mid;
            } else {
                max_val = mid;
            }
            error = f64::abs(prev_mid - mid);
            prev_mid = mid;
        }
        min_val
    }

    fn has_k_more_average(nums: &[i32], ave: f64, k: usize) -> bool {
        let mut min_sum = 0f64;
        let mut sum = 0f64;
        for i in 0..k {
            sum += (nums[i] as f64) - ave;
        }
        if sum > 0f64 {
            return true;
        }
        let mut prev = 0f64;
        for i in k..nums.len() {
            sum += (nums[i] as f64) - ave;
            prev += (nums[i - k] as f64) - ave;
            min_sum = f64::min(prev, min_sum);
            if sum - min_sum > 0.0 {
                return true;
            }
        }
        false
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn test_find_max_average_1() {
        assert_feq!(Solution::find_max_average(vec![1,12,-5,-6,50,3], 4), 12.75, 1e-5);
    }
}