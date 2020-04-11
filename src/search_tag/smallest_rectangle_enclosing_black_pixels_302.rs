/*
 * @lc app=leetcode.cn id=302 lang=rust
 *
 * [302] 包含全部黑色像素的最小矩形
 */
// @lc code=start
impl Solution {
    pub fn min_area(image: Vec<Vec<char>>, _x: i32, _y: i32) -> i32 {
        if image.is_empty() {
            return 0;
        }   
        if image[0].is_empty() {
            return 0;
        }
        let row_len = image.len();
        let col_len: usize = image[0].len();
        let mut rows = vec![false; row_len];
        let mut cols = vec![false; col_len];
        for i in 0..row_len {
            for j in 0..col_len {
                rows[i] = rows[i] || image[i][j]=='1';
                cols[j] = cols[j] || image[i][j]=='1';
            }
        }
        let height = rows.into_iter().filter(|r| *r).count();
        let width = cols.into_iter().filter(|c| *c).count();
        (height * width) as i32
    }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_area() {
        let input = [
            "0010",
            "0110",
            "0100"
          ].iter().map(|s| s.chars().collect::<Vec<char>>())
          .collect();
        assert_eq!(Solution::min_area(input, 0, 2), 6);
    }
}