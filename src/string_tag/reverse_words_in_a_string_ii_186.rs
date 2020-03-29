/*
 * @lc app=leetcode.cn id=186 lang=rust
 *
 * [186] 翻转字符串里的单词 II
 */
struct Solution;
// @lc code=start

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        if s.is_empty() {
            return;
        }
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            while j+1 < s.len() && s[j+1]!=' ' {
                j += 1;
            }
            let new_i = if j+1 >= s.len() {
                j+1
            } else {
                j+2
            };
            while i < j {
                s.swap(i,j);
                i+=1;
                j-=1;
            }
            i = new_i;
        }
        let mut j = 0;
        let mut k = s.len() - 1;
        while j < k {
            s.swap(j,k);
            j+=1;
            k-=1;
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let mut src = vec!['t','h','e',' ','s','k','y',' ','i','s',' ','b','l','u','e'];
        let tar = vec!['b','l','u','e',' ','i','s',' ','s','k','y',' ','t','h','e'];
        Solution::reverse_words(&mut src);
        assert_eq!(src, tar);
    }
}