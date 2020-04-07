/*
 * @lc app=leetcode.cn id=289 lang=rust
 *
 * [289]  搜寻名人
 */
// @lc code=start
impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut i = 0;
        if nums.len() < 2 {
            return;
        }
        while i+1<nums.len() {
            if (i&1)==0 {
                if nums[i]>nums[i+1] {
                    nums.swap(i, i+1);
                }
            } else {
                if nums[i]<nums[i+1] {
                    nums.swap(i, i+1);
                }
            }
            i+=1;
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiggle_sort() {
        let mut nums = vec![3,5,2,1,6,4];
        Solution::wiggle_sort(&mut nums);
        assert_eq!(nums, vec![3,5,1,6,2,4]);
    }
}