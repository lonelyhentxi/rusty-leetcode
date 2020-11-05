/*
 * @lc app=leetcode.cn id=616 lang=rust
 *
 * [616] 给字符串添加加粗标签
 */
// @lc code=start
const OPEN_TAG: &'static str = "<b>";
const CLOSE_TAG: &'static str = "</b>";

impl Solution {
    pub fn add_bold_tag(s: String, dict: Vec<String>) -> String {
        let mut ranges = vec![];
        let len = s.len();
        if dict.is_empty() {
            return s;
        }
        for d in dict {
            let dlen = d.len();
            if len >= dlen {
                for i in 0..(len - dlen + 1) {
                    if &d == &s[i..(i + dlen)] {
                        ranges.push((i, i + dlen));
                    }
                }
            }
        }
        ranges.sort();
        let mut merged: Vec<(usize, usize)> = vec![];
        for r in ranges {
            let last = merged.pop();
            if let None = last {
                merged.push(r);
            } else {
                let l = last.unwrap();
                if l.1 >= r.0 {
                    merged.push((l.0, usize::max(l.1, r.1)));
                } else {
                    merged.push(l);
                    merged.push(r);
                }
            }
        }
        if merged.is_empty() {
            return s;
        }

        let mut res = String::new();
        for i in 0..merged.len() {
            let curr = &merged[i];
            if i >= 1 {
                let last = &merged[i - 1];
                res += &s[last.1..curr.0];
            } else {
                res += &s[0..curr.0];
            }
            res += OPEN_TAG;
            res += &s[curr.0..curr.1];
            res += CLOSE_TAG;
        }
        if let Some(&(_, to)) = merged.last() {
            res += &s[to..];
        }
        res
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    fn test_add_bold_tag_1() {
        assert_eq!(Solution::add_bold_tag(String::from("abcxyz123"), lc_vec_s!["abc","123"]), String::from("<b>abc</b>xyz<b>123</b>"));
    }

    #[test]
    fn test_add_bold_tag_2() {
        assert_eq!(Solution::add_bold_tag(String::from("aaabbcc"), lc_vec_s!["aaa","aab","bc"]), String::from("<b>aaabbc</b>c"));
    }
}