/*
 * @lc app=leetcode.cn id=311 lang=rust
 *
 * [311] 稀疏矩阵的乘法
 */

// @lc code=start
use std::collections::hash_map::HashMap;

enum MatrixAxis {
    ROW,
    COL,
}

impl Solution {
    pub fn multiply(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let a_rows = a.len();
        let multi_lines = b.len();
        let b_cols = if multi_lines == 0 { 0 } else { b[0].len() };
        let a_col_numbers = Solution::build_axis_numbers_from_matrix(a, MatrixAxis::COL);
        println!("{:?}", a_col_numbers);
        let b_row_numbers = Solution::build_axis_numbers_from_matrix(b, MatrixAxis::ROW);
        if a_rows == 0 || multi_lines == 0 || b_cols == 0 {
            return vec![];
        }
        let mut res = vec![vec![0; b_cols]; a_rows];
        for k in 0..multi_lines {
            if !a_col_numbers.contains_key(&k) || !b_row_numbers.contains_key(&k) {
                continue;
            }
            for (&i, &av) in &a_col_numbers[&k] {
                for (&j, &bv) in &b_row_numbers[&k] {
                    res[i][j] += av * bv;
                }
            }
        }
        res
    }

    fn build_axis_numbers_from_matrix(
        matrix: Vec<Vec<i32>>,
        axis: MatrixAxis,
    ) -> HashMap<usize, HashMap<usize, i32>> {
        match axis {
            MatrixAxis::ROW => matrix
                .into_iter()
                .enumerate()
                .map(|(i, r)| {
                    (
                        i,
                        r.into_iter()
                            .enumerate()
                            .filter(|(_, v)| *v != 0)
                            .collect::<HashMap<usize, i32>>(),
                    )
                })
                .filter(|(_, v)| !v.is_empty())
                .collect::<HashMap<usize, HashMap<usize, i32>>>(),
            MatrixAxis::COL => {
                let rows = matrix.len();
                let cols = if rows == 0 { 0 } else { matrix[0].len() };
                (0..cols)
                    .into_iter()
                    .map(|j| {
                        (
                            j,
                            (0..rows)
                                .into_iter()
                                .map(|i| matrix[i][j])
                                .enumerate()
                                .filter(|(_, v)| *v != 0)
                                .collect::<HashMap<usize, i32>>(),
                        )
                    })
                    .filter(|(_, v)| !v.is_empty())
                    .collect::<HashMap<usize, HashMap<usize, i32>>>()
            }
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = vec![vec![1, 0, 0], vec![-1, 0, 3]];
        let b = vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(
            Solution::multiply(a, b),
            vec![vec![7, 0, 0], vec![-7, 0, 3]]
        );
    }
}
