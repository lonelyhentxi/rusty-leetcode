/*
 * @lc app=leetcode.cn id=252 lang=rust
 *
 * [252] 会议室
 */

// @lc code=start


impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by_key(|v| v[0]);
        if intervals.is_empty() {
            return true;
        }
        let mut all_period = intervals[0].clone();
        for i in 1..intervals.len() {
            let period = &intervals[i];
            if period[0] < all_period[1] {
                return false;
            } else {
                all_period[1] = period[1];
            }
        }
        true
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test  {
    use super::Solution;

    #[test]
    fn test_can_attend_meetings_1() {
        let src = vec![vec![0,30],vec![5,10],vec![15,20]];
        assert!(!Solution::can_attend_meetings(src));
    }

    #[test]
    fn test_can_attend_meetings_2() {
        let src = vec![vec![7,10],vec![2,4]];
        assert!(Solution::can_attend_meetings(src));
    }
}

