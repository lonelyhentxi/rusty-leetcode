use std::vec;

/*
 * @lc app=leetcode.cn id=308 lang=rust
 *
 * [308] 二维区域和检索 - 可变
 */

// @lc code=start
#[derive(Debug)]
struct NumMatrix {
    tree: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let rows = matrix.len();
        let cols = if rows <= 0 { 0 } else { matrix[0].len() };
        let tree = vec![vec![0; cols + 1]; rows + 1];
        let clone = vec![vec![0; cols]; rows];
        let mut res = NumMatrix {
            tree,
            cols,
            rows,
            matrix: clone
        };
        for i in 0..rows {
            for j in 0..cols {
                res.update(i as i32, j as i32, matrix[i][j]);
            }
        }
        res
    }
    pub fn update(&mut self, row: i32, col: i32, val: i32) {
        let delta = val - self.matrix[row as usize][col as usize];
        self.matrix[row as usize][col as usize] = val;
        let mut i = (row + 1) as usize;
        while i <= self.rows {
            let mut j = (col + 1) as usize;
            while j <= self.cols {
                self.tree[i][j] += delta;
                j += NumMatrix::low_bit(j as i32) as usize;
            }
            i += NumMatrix::low_bit(i as i32) as usize;
        }
    }
    
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.get_sum(row2 + 1, col2 + 1)
            - self.get_sum(row1, col2 + 1)
            - self.get_sum(row2 + 1, col1)
            + self.get_sum(row1, col1)
    }

    fn low_bit(num: i32) -> i32 {
        num & (-num)
    }

    fn get_sum(&self, row: i32, col: i32) -> i32 {
        let mut sum = 0;
        let mut i = row;
        while i > 0 {
            let mut j = col;
            while j > 0 {
                sum += self.tree[i as usize][j as usize];
                j -= NumMatrix::low_bit(j);
            }
            i -= NumMatrix::low_bit(i);
        }
        sum
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_sum_query_2d_mutable() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        println!("{:?}", &num_matrix);
        assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 8);
        num_matrix.update(3, 2, 2);
        assert_eq!(num_matrix.sum_region(2, 1, 4, 3), 10);
    }
}
