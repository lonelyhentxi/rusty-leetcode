/*
 * @lc app=leetcode.cn id=750 lang=rust
 *
 * [750] 角矩形的数量
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn count_corner_rectangles(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };
        if rows * cols == 0 {
            return 0;
        }
        let mut dict = HashMap::<(usize, usize), usize>::new();
        for i in 0..rows {
            let mut exists = grid[i]
                .iter()
                .enumerate()
                .filter(|(_, &v)| v != 0)
                .map(|(j, _)| j)
                .collect::<Vec<_>>();
            let e_len = exists.len();
            if e_len <= 1 {
                continue;
            }
            exists.sort();
            for i in 0..e_len {
                for j in (i + 1)..e_len {
                    dict.entry((exists[i], exists[j])).and_modify(|v| *v += 1).or_insert(1);
                }
            }
        }
        dict.into_iter()
            .map(|(_, v)| v * (v - 1) / 2)
            .fold(0usize, |acc, curr| acc + curr) as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_count_corner_rectangles_1() {
        let grid = lc_matrix![
            [1, 0, 0, 1, 0],
            [0, 0, 1, 0, 1],
            [0, 0, 0, 1, 0],
            [1, 0, 1, 0, 1]
        ];
        assert_eq!(Solution::count_corner_rectangles(grid), 1);
    }

    #[test]
    fn test_count_corner_rectangles_2() {
        let grid = lc_matrix![[1, 1, 1], [1, 1, 1], [1, 1, 1]];
        assert_eq!(Solution::count_corner_rectangles(grid), 9);
    }

    #[test]
    fn test_count_corner_rectangles_3() {
        let grid = lc_matrix![[1, 1, 1, 1]];
        assert_eq!(Solution::count_corner_rectangles(grid), 0);
    }
}
