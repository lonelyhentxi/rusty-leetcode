/*
 * @lc app=leetcode.cn id=317 lang=rust
 *
 * [317] 离建筑物最近的距离
 */

// @lc code=start
use std::collections::VecDeque;

const DELTAS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn shortest_distance(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };
        if rows == 0 || cols == 0 {
            return -1;
        }
        let mut accessible = vec![vec![0; cols]; rows];
        let mut cost = vec![vec![0; cols]; rows];

        let mut buildings = 0;
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    let mut deque = VecDeque::<(isize, isize, i32)>::new();
                    deque.push_back((i as isize, j as isize, 0));
                    accessible[i][j] = buildings;
                    while let Some((x , y, c)) = deque.pop_front() {
                        let acc = accessible[x as usize][y as usize];
                        accessible[x as usize][y as usize] = 
                            if acc == buildings {
                                for (dx, dy) in &DELTAS {
                                    let nx = x + dx;
                                    let ny = y + dy;
                                    if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize || grid[nx as usize][ny as usize] != 0 {
                                        continue
                                    }
                                    deque.push_back((nx, ny, c + 1));
                                }
                                cost[x as usize][y as usize] += c;
                                buildings + 1
                            } else if acc > buildings {
                                acc
                            } else {
                                -1
                            }
                    }
                    buildings += 1;
                }
            }
        }
        let mut md = i32::max_value();
        for i in 0..rows {
            for j in 0..cols {
                md = if accessible[i][j] == buildings && grid[i][j]==0 { i32::min(cost[i][j], md) } else { md };
            }
        }
        if md == i32::max_value() { -1 } else { md }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_shortest_distance() {
        let grid = vec![vec![1,0,2,0,1],vec![0,0,0,0,0], vec![0,0,1,0,0]];
        assert_eq!(Solution::shortest_distance(grid), 7);
    }
}
