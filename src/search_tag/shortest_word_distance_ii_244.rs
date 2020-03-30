/*
 * @lc app=leetcode.cn id=244 lang=rust
 *
 * [244] 最短单词距离 II
 */
// @lc code=start
use std::collections::HashMap;

struct WordDistance {
    dict: HashMap<String,Vec<usize>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDistance {

    fn new(words: Vec<String>) -> Self {
        let mut dict: HashMap<String,Vec<usize>> = HashMap::new();
        for (i, w) in words.iter().cloned().enumerate() {
            dict.entry(w)
                .and_modify(|vs: &mut Vec<usize>| vs.push(i))
                .or_insert_with(|| vec![i]);
        }
        WordDistance {
            dict
        }
    }
    
    fn shortest(&self, word1: String, word2: String) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let indices1 = self.dict.get(&word1).unwrap();
        let indices2 = self.dict.get(&word2).unwrap();
        let mut dist = i32::max_value();
        while i<indices1.len() && j< indices2.len() {
            let i_value = indices1[i] as i32;
            let j_value = indices2[j] as i32;
            dist = i32::min(dist, i32::abs(i_value - j_value));
            if i_value < j_value {
                i += 1;
            } else {
                j += 1;
            }
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
    fn test_word_distance_ii() {
        let words = map_to_string(&["practice", "makes", "perfect", "coding", "makes"]);
        let wd = WordDistance::new(words);
        assert_eq!(wd.shortest(String::from("coding"), String::from("practice")), 3);
        assert_eq!(wd.shortest(String::from("makes"), String::from("coding")), 1);
    }
}