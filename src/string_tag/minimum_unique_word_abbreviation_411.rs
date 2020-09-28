/*
 * @lc app=leetcode.cn id=411 lang=rust
 *
 * [411] 最短特异单词缩写
 */

// @lc code=start
impl Solution {
    pub fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
        let chars = target.chars().collect::<Vec<char>>();
        let len = chars.len();
        if len <= 0 {
            return target;
        }
        let size = 1usize << len;
        let dic_same = dictionary
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .filter(|v| v.len() == len)
            .map(|d| {
                let mut same = 0;
                for i in 0..len {
                    same += if d[i] != chars[i] { 1usize << i } else { 0 }
                }
                same
            })
            .collect::<Vec<usize>>();
        if dic_same.len() == 0 {
            return len.to_string();
        }
        
        let factors = (0..len).map(|i| 1 << i).collect::<Vec<usize>>();

        let mut min_len = usize::max_value();
        let mut abbr = 0;
        'outer: for i in 0..size {
            for d in &dic_same {
                if d & i == 0 {
                    continue 'outer;
                }
            }
            let abbr_len = Solution::abbr_to_len(i, &factors);
            if abbr_len < min_len {
                min_len = abbr_len;
                abbr = i;
            }
        }
        match abbr {
            0 => target,
            i => Solution::abbr_to_string(i, &chars, &factors),
        }
    }

    fn abbr_to_string(i: usize, chars: &Vec<char>, factors: &Vec<usize>) -> String {
        let mut curr = String::new();
        let mut num: i32 = 0;
        for j in 0..chars.len() {
            if i & factors[j] == 0 {
                num += 1;
            } else {
                if num != 0 {
                    curr += &num.to_string();
                    num = 0;
                }
                curr.push(chars[j]);
            }
        }
        if num != 0 {
            curr += &num.to_string();
        }
        return curr;
    }

    fn abbr_to_len(i: usize, factors: &Vec<usize>) -> usize {
        let mut last_is_char = false;
        let mut num = factors.len();
        for j in 0..factors.len() {
            if i & factors[j] == 0 {
                if last_is_char {
                    num += 1;
                }
                num -= 1;
                last_is_char = false;
            } else {
                last_is_char = true;
            }
        }
        num
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_min_abbreviation_1() {
        assert_eq!(
            Solution::min_abbreviation(String::from("apple"), vec![String::from("blade")]),
            String::from("a4")
        );
    }

    #[test]
    fn test_min_abbreviation_2() {
        let set = vec!["ap3", "a3e", "2p2", "3le", "3l1", "1p3"]
            .into_iter()
            .map(String::from)
            .collect::<HashSet<String>>();
        assert!(set.contains(&Solution::min_abbreviation(
            String::from("apple"),
            vec![
                String::from("plain"),
                String::from("amber"),
                String::from("blade")
            ]
        )));
    }
}
