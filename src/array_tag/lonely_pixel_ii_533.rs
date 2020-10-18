/*
 * @lc app=leetcode.cn id=533 lang=rust
 *
 * [533] 孤独像素 II
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn find_black_pixel(picture: Vec<Vec<char>>, n: i32) -> i32 {
        let rows = picture.len();
        let cols = if rows == 0 { 0 } else { picture[0].len() };
        if rows == 0 || cols == 0 {
            return 0;
        };
        let mut row_memo = HashMap::<Vec<char>, usize>::new();
        let mut row_b = vec![0; rows];
        let mut col_b = vec![0; cols];
        for i in 0..rows {
            for j in 0..cols {
                if picture[i][j] == 'B' {
                    col_b[j] += 1;
                    row_b[i] += 1;
                }
            }
            if row_b[i] == n { 
                row_memo.entry(picture[i].clone()).and_modify(|v| *v+=1).or_insert_with(|| 1);
            }
        }
        let mut sum = 0;
        for (k, v) in row_memo.into_iter() {
            if v == (n as usize) {
                for j in 0..cols {
                    if k[j] == 'B' && col_b[j] == n {
                        sum += n;
                    }
                }
            }
        }
        sum
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_black_pixel_1() {
        let p = vec![
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'B', 'W', 'B', 'B', 'W'],
            vec!['W', 'W', 'B', 'W', 'B', 'W'],
        ];
        assert_eq!(Solution::find_black_pixel(p, 3), 6);
    }
}
