/*
 * @lc app=leetcode.cn id=293 lang=rust
 *
 * [293] 翻转游戏
 */

// @lc code=start
impl Solution {
    pub fn generate_possible_next_moves(s: String) -> Vec<String> {
        let chars = s.chars().collect::<Vec<char>>();
        let mut res = vec![];
        for i in 1..chars.len() {
            let curr = chars[i];
            let prev = chars[i-1];
            if curr=='+' && prev=='+' {
                let mut new_chars = chars.clone();
                new_chars[i-1] = '-';
                new_chars[i] = '-';
                res.push(new_chars.into_iter().collect::<String>());
            }
        }
        res
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::{map_to_string,assert_equivalent};

    #[test]
    fn test_generate_possible_next_moves() {
        let tar = map_to_string(&["--++","+--+","++--"]);
        assert_equivalent(
            &Solution::generate_possible_next_moves(
                String::from("++++")), 
            &tar);
    }
}