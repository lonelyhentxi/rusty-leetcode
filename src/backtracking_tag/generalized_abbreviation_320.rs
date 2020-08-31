/*
 * @lc app=leetcode.cn id=320 lang=rust
 *
 * [320] 列举单词的全部缩写
 */

// @lc code=start
use std::collections::hash_set::HashSet;

impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let chars = word.chars().collect::<Vec<char>>();
        let len = chars.len();
        let size = 1 << len;
        let factors = (0..len).map(|f| 1 << f).collect::<Vec<usize>>();
        (0..size)
            .map(|i| {
                let mut word = chars.clone();
                for j in 0..len {
                    if i & factors[j] == 0 {
                        word[j] = '#';
                    }
                }
                let mut new_word = String::new();
                let mut count = 0;
                for j in 0..len {
                    if word[j] == '#' {
                        count += 1;
                    } else {
                        if count != 0 {
                            new_word += &count.to_string();
                        }
                        new_word.push(word[j]);
                        count = 0;
                    }
                }
                if count != 0 {
                    new_word += &count.to_string();
                }
                new_word
            })
            .collect()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {

    use super::*;
    use crate::utils::test_tools::assert_equivalent;

    #[test]
    fn test_generate_abbreviations() {
        let word = String::from("word");
        assert_equivalent(
            &Solution::generate_abbreviations(word),
            &(vec![
                "word", "1ord", "w1rd", "wo1d", "wor1", "2rd", "w2d", "wo2", "1o1d", "1or1",
                "w1r1", "1o2", "2r1", "3d", "w3", "4",
            ])
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>(),
        );
    }
}
