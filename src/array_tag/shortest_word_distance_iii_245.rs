/*
 * @lc app=leetcode.cn id=245 lang=rust
 *
 * [245] 最短单词距离 III
 */
struct Solution;
// @lc code=start
impl Solution {
    pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut i = -1 - (words.len() as i32);
        let mut j = -1 - (words.len() as i32);
        let mut dist = i32::max_value();
        let is_same = word1 == word2;
        for (k, w) in words.iter().enumerate() {
            if is_same && *w==word1 {
                if i < j {
                    i = k as i32;
                } else {
                    j = k as i32;
                }
            } else if *w == word1 {
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
    fn test_shortest_distance_iii_1() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(Solution::shortest_word_distance(words, String::from("coding"), String::from("practice")), 3);
    }

    
    #[test]
    fn test_shortest_distance_iii_2() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(Solution::shortest_word_distance(words, String::from("makes"), String::from("coding")), 1);
    }

    #[test]
    fn test_shortest_distance_iii_3() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        assert_eq!(Solution::shortest_word_distance(words, String::from("makes"), String::from("makes")), 3);
    }
}