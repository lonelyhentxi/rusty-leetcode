/*
 * @lc app=leetcode.cn id=161 lang=rust
 *
 * [161] 相隔为 1 的编辑距离
 */
struct Solution;
// @lc code=start

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let s_chars = s.chars().collect::<Vec<char>>();
        let t_chars = t.chars().collect::<Vec<char>>();
        let s_len = s.len();
        let t_len = t.len();
        if i32::abs((s_len as i32) - (t_len as i32)) > 1 {
            false
        } else if s_len == t_len {
            let mut count = 0;
            for i in 0..s_len {
                if s_chars[i]!=t_chars[i] {
                    count += 1;
                }
            }
            count == 1
        } else if s_len==0 || t_len==0 {
            true
        } else {
            let mut i = 0;
            while i<s_len && i<t_len && s_chars[i]==t_chars[i] {
                i+=1;
            }
            let mut j = i+1;
            let (longer_chars, shorter_chars) = if s_len > t_len {
                (&s_chars, &t_chars)
            } else {
                (&t_chars, &s_chars)
            };
            while i<shorter_chars.len() && j<longer_chars.len() {
                if shorter_chars[i]!=longer_chars[j] {
                    return false;
                }
                i+=1;
                j+=1;
            }
            true
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_edit_distance() {
        assert!(
            Solution::is_one_edit_distance(
                String::from("ab"),
                String::from("acb")
            )
        );
        assert!(
            !Solution::is_one_edit_distance(
                String::from("cab"),
                String::from("ad"),
            )
        );
        assert!(
            Solution::is_one_edit_distance(
                String::from("1203"),
                String::from("1213")
            )
        );
    }
}