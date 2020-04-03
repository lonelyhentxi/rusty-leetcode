/*
 * @lc app=leetcode.cn id=259 lang=rust
 *
 * [259] 较小的三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum_smaller(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        if nums.len() < 3 {
            return 0;
        }
        let mut count = 0i32;
        for i in (2..nums.len()).rev() {
            let threshold = target - nums[i];
            let mut j= 0;
            let mut k  = i-1;
            while j<k {
                if nums[j] + nums[k] < threshold {
                    count += (k-j) as i32;
                    j+=1;
                } else {
                    k-=1;
                }
            }
        }
        count
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum_smaller() {
        let nums = vec![-2,0,1,3];
        assert_eq!(Solution::three_sum_smaller(nums, 4), 3);
    }
}