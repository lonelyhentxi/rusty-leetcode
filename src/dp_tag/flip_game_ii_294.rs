/*
 * @lc app=leetcode.cn id=294 lang=rust
 *
 * [294] 翻转游戏 II
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn can_win(s: String) -> bool {
        let mut memo = HashMap::new();
        Solution::can_win_rec(&mut memo, s)
    }

    fn can_win_rec(
        memo: &mut HashMap<String, bool>, 
        s: String
    )  -> bool
    {
        if let Some(res) = memo.get(&s) {
            return *res;
        }
        let len = s.len();
        for i in 1..len {
            let chars = s.chars().collect::<Vec<char>>();
            let prev = chars[i-1];
            let curr = chars[i];
            if prev=='+' && curr=='+' {
                let mut new_target = chars.to_vec();
                new_target[i-1] = '-';
                new_target[i] = '-';
                let new_target= new_target.into_iter().collect::<String>();
                if !Solution::can_win_rec(
                    memo, new_target
                    ) {
                    memo.insert(s, true);
                    return true;
                }
            }
        }
        memo.insert(s, false);
        false
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_can_win() {
        assert!(Solution::can_win(String::from("++++")));
    }
}

