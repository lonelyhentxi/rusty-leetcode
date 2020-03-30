/*
 * @lc app=leetcode.cn id=243 lang=rust
 *
 * [243] 最短单词距离
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut i = -1 - (words.len() as i32);
        let mut j = -1 - (words.len() as i32);
        let mut dist = i32::max_value();
        for (k, w) in words.iter().enumerate() {
            if *w == word1 {
               i = k as i32;
            } else if *w == word2 {
               j = k as i32;
            } else {
                continue;
            }
            dist = i32::min(dist, i32::abs(i-j));
        }
        dist
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_tools::map_to_string;

    #[test]
    fn test_shortest_distance1() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(Solution::shortest_distance(words, String::from("coding"), String::from("practice")), 3);
    }

    
    #[test]
    fn test_shortest_distance2() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(Solution::shortest_distance(words, String::from("makes"), String::from("coding")), 1);
    }
}