/*
 * @lc app=leetcode.cn id=418 lang=rust
 *
 * [418] 屏幕可显示句子的数量
 */

// @lc code=start
impl Solution {
    pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
        let chars = sentence.into_iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
        let len = chars.len();
        let cols = cols as usize;
        let rows = rows as usize;
        if len == 0 || rows == 0 || cols == 0 {
            return 0;
        }
        let cols = cols + 1;
        let lens = chars.iter().map(|c| c.len() + 1).collect::<Vec<_>>();
        let mut count = 0usize;
        let mut si = 0usize;
        let mut r = 0usize;
        let mut dp: Vec<Option<(usize, usize)>> = vec![None; len];
        while r < rows {
            if si == 0 && r != 0 && rows >= r * 2 {
                let times = rows / r;
                r = times * r;
                count = count * times;
            } else {
                let fsi = si;
                let fcount = count;
                if let Some((nsi, inc)) = dp[fsi] {
                    si = nsi;
                    count += inc;
                } else {
                    let mut c = 0usize;
                    loop {
                        let w = lens[si];
                        let nc = c + w;
                        if nc > cols {
                            break;
                        }
                        c = nc;
                        si += 1;
                        if si >= len {
                            si = 0;
                            count += 1;
                        }
                    }
                    dp[fsi] = Some((si, count - fcount));
                }
                r += 1;
            }
        }
        count as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_words_typing_1() {
        assert_eq!(Solution::words_typing(vec![String::from("hello"), String::from("world")], 2, 8), 1);
    }

    #[test]
    fn test_words_typing_2() {
        assert_eq!(Solution::words_typing(vec![String::from("a"), String::from("bcd"), String::from("e")], 3, 6), 2);
    }

    #[test]
    fn test_words_typing_3() {
        assert_eq!(Solution::words_typing(vec![String::from("I"), String::from("had"), String::from("apple"), String::from("pie")], 4, 5), 1);
    }

    #[test]
    fn test_words_typing_4() {
        assert_eq!(Solution::words_typing(vec![String::from("f"), String::from("p"), String::from("a")], 8, 7), 10);
    }

    #[test]
    fn test_words_typing_5() {
        assert_eq!(Solution::words_typing(vec![String::from("a")], 2, 3), 4);
    }
}

