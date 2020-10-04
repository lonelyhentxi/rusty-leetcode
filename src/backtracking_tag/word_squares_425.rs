/*
 * @lc app=leetcode.cn id=425 lang=rust
 *
 * [425] 单词方块
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let len = words[0].len();
        let mut cache = HashMap::<&str, Vec<usize>>::new();
        for (i, w) in words.iter().enumerate() {
            for j in 0..len {
                cache
                    .entry(&w[0..j])
                    .and_modify(|v| v.push(i))
                    .or_insert(vec![i]);
            }
        }
        let mut tries: Vec<(usize, String, Vec<usize>)> = vec![(0, String::new(), vec![])];
        let mut res = vec![];
        while let Some((i, pre, used)) = tries.pop() {
            let ni = i + 1;
            for k in &cache[&pre[..]] {
                let mut used_copy = used.clone();
                used_copy.push(*k);
                if ni >= len {
                    res.push(used_copy);
                } else {
                    let next_pre = String::from_utf8_lossy(&used_copy.iter().map(|v| words[*v].as_bytes()[ni]).collect::<Vec<u8>>()).to_string();
                    if cache.contains_key(&next_pre[..]) {
                        tries.push((ni, next_pre, used_copy));
                    }
                }
            }
        }
        res.into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|i| &words[i])
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<String>>>()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::{ map_nested_to_string, map_to_string, assert_equivalent };

    #[test]
    fn test_word_squares_1() {
        let input = map_to_string(&[
            "area","lead","wall","lady","ball"
        ]);
        let output = map_nested_to_string(
            &[
                vec![ "wall",
                  "area",
                  "lead",
                  "lady"
                ],
                vec![ "ball",
                  "area",
                  "lead",
                  "lady"
                ]
              ]  
        );
        assert_equivalent(&Solution::word_squares(input), &output);
    }

    #[test]
    fn test_word_squares_2() {
        let input = map_to_string(&[
            "abat","baba","atan","atal"
        ]);
        let output = map_nested_to_string(
            &[
                vec![
                "baba",
                "abat",
                "baba",
                "atan"
              ],
              vec![
                "baba",
                "abat",
                "baba",
                "atal"
              ]
            ]
        );
        assert_equivalent(&Solution::word_squares(input), &output);
    }
}
