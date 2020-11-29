/*
 * @lc app=leetcode.cn id=734 lang=rust
 *
 * [734] 句子相似性
 */

// @lc code=start
use std::collections::HashSet;

impl Solution {
    pub fn are_sentences_similar(
        sentence1: Vec<String>,
        sentence2: Vec<String>,
        similar_pairs: Vec<Vec<String>>,
    ) -> bool {
        let sps = similar_pairs
            .iter()
            .map(|sp| (&sp[0] as &str, &sp[1] as &str))
            .collect::<HashSet<(&str, &str)>>();
        let len1 = sentence1.len();
        let len2 = sentence2.len();
        if len1 != len2 {
            return false;
        } else {
            for i in 0..len1 {
                let w1 = &sentence1[i] as &str;
                let w2 = &sentence2[i] as &str;
                if w1 != w2 {
                    let p1 = &(w1, w2);
                    let p2 = &(w2, w1);
                    if !sps.contains(&p1) && !sps.contains(p2) {
                        return false;
                    }
                }
            }
            true
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lc_matrix_s, lc_vec_s};

    #[test]
    fn test_are_sentences_similar_1() {
        assert_eq!(
            Solution::are_sentences_similar(
                lc_vec_s!["great", "haha"],
                lc_vec_s!["fine", "haha"],
                lc_matrix_s![["great", "fine"], ["drama", "acting"], ["skills", "talent"]]
            ),
            true
        );
    }
}
