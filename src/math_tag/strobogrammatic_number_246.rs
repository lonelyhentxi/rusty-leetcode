/*
 * @lc app=leetcode.cn id=246 lang=rust
 *
 * [246] 中心对称数
 */
struct Solution;
// @lc code=start
impl Solution {
    #[inline]
    fn is_center(c: char) -> bool {
        c=='0' || c=='8' || c=='1'
    }

    #[inline]
    fn is_pair(c1: char, c2: char) -> bool {
        (c1=='6' && c2=='9') ||
        (c1=='9' && c2=='6') ||
        (c1==c2 && Solution::is_center(c1))
    }

    pub fn is_strobogrammatic(num: String) -> bool {
        if num.is_empty() {
            return true;
        }
        let nums = num.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i<j {
            if !Solution::is_pair(nums[i], nums[j]) {
                return false;
            }
            i+=1;
            j-=1;
        }
        if i==j {
            Solution::is_center(nums[i])
        } else {
            true
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_strobogrammatic() {
        assert!(Solution::is_strobogrammatic(String::from("69")));
        assert!(Solution::is_strobogrammatic(String::from("88")));
        assert!(!Solution::is_strobogrammatic(String::from("692")));
    }
}