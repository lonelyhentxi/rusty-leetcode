/*
 * @lc app=leetcode.cn id=361 lang=rust
 *
 * [361] 轰炸敌人
 */

// @lc code=start
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn max_killed_enemies(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };
        let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0usize; cols + 2]; rows + 2]; 4];
        for k in 0..4 {
            let (di, dj) = DIRECTIONS[k];
            let is: Vec<usize> = if di > 0 { (0..rows).rev().collect() } else { (0..rows).collect() };
            let js: Vec<usize>  = if dj > 0 { (0..cols).rev().collect() } else { (0..cols).collect() }; 
            for &i in &is {
                for &j in &js {
                    let ti = i + 1;
                    let tj = j + 1;
                    let ni = (ti as isize + di) as usize;
                    let nj  = (tj as isize + dj) as usize;
                    dp[k][ti][tj] = match grid[i][j] {
                        'W' => 0,
                        'E' => dp[k][ni][nj] + 1,
                        _ => dp[k][ni][nj]
                    }
                }
            }
        }
        let mut max = 0;
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '0' {
                    let ti = i + 1;
                    let tj = j + 1;
                    max = i32::max(max, (dp[0][ti][tj] + dp[1][ti][tj] + dp[2][ti][tj] + dp[3][ti][tj]) as i32);
                }
            }
        }
        max
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bomb_enemy() {
        let matrix = vec![
            vec!['0','E','0','0'],
            vec!['E','0','W','E'],
            vec!['0','E','0','0']
        ];
        assert_eq!(Solution::max_killed_enemies(matrix), 3);
    }
}