/*
 * @lc app=leetcode.cn id=531 lang=rust
 *
 * [531] 孤独像素 I
 */

// @lc code=start
impl Solution {
    pub fn find_lonely_pixel(picture: Vec<Vec<char>>) -> i32 {
        let rows = picture.len();
        let cols = if rows == 0 { 0 } else { picture[0].len() };
        if rows == 0 || cols == 0 {
            return 0;
        };
        let mut row_b = vec![(0usize, 0usize); rows];
        let mut col_b = vec![0usize; cols];
        for i in 0..rows {
            for j in 0..cols {
                if picture[i][j] == 'B' {
                    row_b[i] = (row_b[i].0 + 1, j);
                    col_b[j] += 1;
                }
            }
        }
        row_b.into_iter().fold(0i32, |acc, (cnt, col)| {
            acc + if cnt == 1 && col_b[col] == 1 { 1 } else { 0 }
        })
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_lonely_pixel_1() {
        assert_eq!(
            Solution::find_lonely_pixel(vec![
                vec!['W', 'W', 'B'],
                vec!['W', 'B', 'W'],
                vec!['B', 'W', 'W']
            ]),
            3
        );
    }
}
