/*
 * @lc app=leetcode.cn id=555 lang=rust
 *
 * [555] 分割连接字符串
 */

// @lc code=start
impl Solution {
    pub fn split_looped_string(strs: Vec<String>) -> String {
        let strs = strs
            .into_iter()
            .map(|s| {
                let rev = s.chars().rev().collect::<String>();
                String::max(rev, s)
            }).collect::<Vec<String>>();
        let size = strs.len();
        let mut res = String::new();
        for i in 0..size {
            let chunk_2 = &strs[(i+1)..].join("");
            let chunk_3 = &strs[0..i].join("");
            for j in 0..strs[i].len() {
                let s = &strs[i];
                let s_rev = &s.chars().rev().collect::<String>();
                {
                    let chunk_1 = &s[j..];
                    let chunk_4 = &s[..j];
                    let joined = vec![chunk_1, chunk_2, chunk_3, chunk_4].join("");
                    res = String::max(res, joined);
                }
                {
                    let chunk_1 = &s_rev[j..];
                    let chunk_4 = &s_rev[..j];
                    let joined = vec![chunk_1, chunk_2, chunk_3, chunk_4].join("");
                    res = String::max(res, joined);
                }
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
    use crate::utils::test_tools::map_to_string;
    
    #[test]
    fn test_split_looped_string_1() {
        assert_eq!(Solution::split_looped_string(
            map_to_string(&["abc", "xyz"])
        ), String::from("zyxcba"));
    }
}