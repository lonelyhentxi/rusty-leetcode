/*
 * @lc app=leetcode.cn id=248 lang=rust
 *
 * [248] 中心对称数 III
 */
struct Solution;
// @lc code=start
use std::collections::HashMap;

const A_CHARCODE: i32 = 'a' as i32;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut dict = HashMap::<String, Vec<String>>::new();
        for s in strings {
            let norm = Solution::shift_normalize(&s);
            dict.entry(norm)
                .and_modify(|v| v.push(s.clone()))
                .or_insert_with(|| vec![s]);
        }
        dict.values().cloned().collect::<Vec<Vec<String>>>()
    }

    fn shift(ch: char, times: i32) -> char {
       (A_CHARCODE + ((ch as i32) - A_CHARCODE + times).rem_euclid(26)) as u8 as char
    }

    fn shift_normalize(string: &String) -> String {
        if string.is_empty() {
            return string.to_string();
        }
        let mut times = 0;
        let chars = string.chars().collect::<Vec<_>>();
        let mut temp_char = chars[0];
        while temp_char!='a' {
            temp_char = Solution::shift(temp_char, -1);
            times -= 1;
        }
        chars
            .iter()
            .map(|ch| Solution::shift(*ch, times))
            .collect::<String>()
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_tools::{ 
        map_to_string,
        map_nested_to_string,
        assert_nested_equivalent,
    };

    #[test]
    fn test_group_strings() {
        let src = map_to_string(&["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"]);
        let tar = map_nested_to_string(&[
            vec!["abc","bcd","xyz"],
            vec!["az","ba"],
            vec!["acef"],
            vec!["a","z"]
        ]);
        assert_nested_equivalent(
            &Solution::group_strings(src), 
            &tar,
        );
    }
}
