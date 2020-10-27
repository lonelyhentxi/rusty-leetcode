/*
 * @lc app=leetcode.cn id=562 lang=rust
 *
 * [562] 矩阵中最长的连续1线段
 */

// @lc code=start
impl Solution {
    pub fn longest_line(m: Vec<Vec<i32>>) -> i32 {
        let rows = m.len();
        let cols = if rows == 0 { 0 } else { m[0].len() };
        let mut dp_vertical = vec![vec![0; cols]; 2];
        let mut dp_main_cross = vec![vec![0; cols]; 2];
        let mut dp_sub_cross = vec![vec![0; cols]; 2];
        let mut max_line = 0;
        for i in 0..rows {
            let mut dp_horizontal = vec![0; 2];
            for j in 0..cols {
                let is_one = m[i][j] == 1;
                let mod_i = i % 2;
                let mod_j = j % 2;
                let mod_prev_i = (i + 1) % 2;
                let mod_prev_j = (j + 1) % 2;
                dp_vertical[mod_i][j] = if is_one {
                    1 + dp_vertical[mod_prev_i][j]
                } else {
                    0
                };
                dp_horizontal[mod_j] = if is_one {
                    1 + dp_horizontal[mod_prev_j]
                } else {
                    0
                };
                dp_main_cross[mod_i][j] = if is_one {
                    1 + if j > 0 {
                        dp_main_cross[mod_prev_i][j - 1]
                    } else {
                        0
                    }
                } else {
                    0
                };
                dp_sub_cross[mod_i][j] = if is_one {
                    1 + if (j + 1) < cols {
                        dp_sub_cross[mod_prev_i][j + 1]
                    } else {
                        0
                    }
                } else {
                    0
                };
                max_line = i32::max(
                    max_line,
                    i32::max(
                        i32::max(dp_vertical[mod_i][j], dp_horizontal[mod_j]),
                        i32::max(dp_main_cross[mod_i][j], dp_sub_cross[mod_i][j]),
                    ),
                )
            }
        }
        max_line
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_line_1() {
        assert_eq!(
            Solution::longest_line(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1]]),
            3
        );
    }

    #[test]
    fn test_longest_line_2() {
        assert_eq!(
            Solution::longest_line(vec![
                vec![0, 1, 0, 1, 1],
                vec![1, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![1, 0, 1, 1, 1],
                vec![1, 0, 0, 0, 1]
            ]),
            3
        )
    }
}
