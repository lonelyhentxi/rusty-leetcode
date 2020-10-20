/*
 * @lc app=leetcode.cn id=544 lang=rust
 *
 * [544] 输出比赛匹配对
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn find_contest_match(n: i32) -> String {
        let mut last_queue = (1..=n).map(|c| c.to_string().chars().collect::<VecDeque<char>>()).collect::<VecDeque<_>>();
        loop {
            let mut curr_queue = VecDeque::new();
            while !last_queue.is_empty() {
                let mut high = last_queue.pop_front().unwrap();
                let low = last_queue.pop_back().unwrap();
                high.push_back(',');
                high.extend(low.into_iter());
                high.push_back(')');
                high.push_front('(');
                curr_queue.push_back(high);
            }
            last_queue = curr_queue;
            if last_queue.len() <= 1 {
                break;
            }
        }
        last_queue.pop_front().map(|s| s.into_iter().collect::<String>()).unwrap_or_else(String::new)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_output_contest_matches_1() {
        assert_eq!(Solution::find_contest_match(2), String::from("(1,2)"));
    }

    #[test]
    fn test_output_contest_matches_2() {
        assert_eq!(Solution::find_contest_match(4), String::from("((1,4),(2,3))"));
    }

    #[test]
    fn test_output_contest_matches_3() {
        assert_eq!(Solution::find_contest_match(8), String::from("(((1,8),(4,5)),((2,7),(3,6)))"));
    }
}