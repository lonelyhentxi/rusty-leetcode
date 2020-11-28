/*
 * @lc app=leetcode.cn id=723 lang=rust
 *
 * [723] 粉碎糖果
 */

// @lc code=start
impl Solution {
    pub fn candy_crush(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = board.len();
        let cols = if rows == 0 { 0 } else { board[0].len() };
        if rows * cols == 0 {
            return board;
        }
        loop {
            let mut crush_size = 0;
            // crush by row
            for i in 0..rows {
                let mut same_count = (0, 0);
                for j in 0..=cols {
                    let current = if j==cols { i32::max_value() } else { board[i][j] };
                    let (last, count) = same_count;
                    if last == i32::abs(current) && current != 0 {
                       same_count.1 += 1;
                    } else {
                        if count >= 3 && last != 0 {
                            for k in (j - count)..j {
                                let b = board[i][k];
                                board[i][k] = if b > 0 {
                                    crush_size += 1;
                                    0 - b
                                } else {
                                    b
                                }
                            }
                        }
                        same_count.0 = i32::abs(current);
                        same_count.1 = 1;
                    }
                }
            }
            // crush by col
            for j in 0..cols {
                let mut same_count = (0, 0);
                for i in 0..=rows {
                    let current = if i==rows { i32::max_value() } else { board[i][j] };
                    let (last, count) = same_count;
                    if last == i32::abs(current) && current != 0 {
                       same_count.1 += 1;
                    } else {
                        if count >= 3 && last != 0 {
                            for k in (i - count)..i {
                                let b = board[k][j];
                                board[k][j] = if b > 0 {
                                    crush_size += 1;
                                    0 - b
                                } else {
                                    b
                                }
                            }
                        }
                        same_count.0 = i32::abs(current);
                        same_count.1 = 1;
                    }
                }
            }
            // drop by col
            for j in 0..cols {
                let mut k = (rows - 1) as i32;
                for i in (0..rows).rev() {
                    let b = board[i][j];
                    if b > 0 {
                        board[k as usize][j] = board[i][j];
                        k -= 1;
                    }
                }
                for i in (0..=k).rev() {
                    board[i as usize][j] = 0;
                }
            }
            if crush_size == 0 {
                break;
            }
        }
        return board;
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_candy_crush_1() {
        let board = lc_matrix![[110,5,112,113,114],[210,211,5,213,214],[310,311,3,313,314],[410,411,412,5,414],[5,1,512,3,3],[610,4,1,613,614],[710,1,2,713,714],[810,1,2,1,1],[1,1,2,2,2],[4,1,4,4,1014]];
        let expect = lc_matrix![[0,0,0,0,0],[0,0,0,0,0],[0,0,0,0,0],[110,0,0,0,114],[210,0,0,0,214],[310,0,0,113,314],[410,0,0,213,414],[610,211,112,313,614],[710,311,412,613,714],[810,411,512,713,1014]];
        let res = Solution::candy_crush(board);
        assert_eq!(expect, res);
    }
}