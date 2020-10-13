/*
 * @lc app=leetcode.cn id=505 lang=rust
 *
 * [505] 迷宫 II
 */

// @lc code=start
use std::collections::{ HashMap, VecDeque };

type Pos = (isize, isize);
const MOVEMENTS: &[Pos; 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

impl Solution {
    pub fn shortest_distance(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> i32 {
        let rows = maze.len() as isize;
        let cols = if rows == 0 { 0 } else { maze[0].len() as isize };
        if rows == 0 || cols == 0 { return -1; };
        let start = (start[0] as isize, start[1] as isize);
        let dest = (destination[0] as isize, destination[1] as isize);
        {
            let (y, x) = start;
            if x < 0 || y < 0 || y >= rows || x >= cols || maze[y as usize][x as usize] != 0 {
                return -1;
            }
        }
        if start == dest { return 0 };
        let mut queue = VecDeque::<Pos>::new();
        let mut memo = HashMap::<Pos, usize>::new();
        queue.push_back(start);
        memo.insert(start, 0);
        let mut min_cost = usize::max_value();
        while let Some(pos) = queue.pop_front() {
            let (y, x) = pos;
            let cost = memo[&pos];
            'movement: for &(dy, dx) in MOVEMENTS {
                let mut cy = y;
                let mut cx = x;
                let mut ccost = cost;
                loop {
                    let ny = cy + dy;
                    let nx = cx + dx;
                    let cpos = (cy, cx);
                    let next_invalid = ny < 0 || nx < 0 || ny >= rows || nx >= cols || maze[ny as usize][nx as usize] != 0;
                    if next_invalid {
                        if !memo.contains_key(&cpos) || memo[&cpos] > ccost  {
                            memo.insert(cpos, ccost);
                            if cpos == dest {
                                min_cost = usize::min(min_cost, ccost);
                            } else {
                                queue.push_back(cpos);
                            }
                        }
                        continue 'movement;
                    }
                    cy = ny;
                    cx = nx;
                    ccost += 1;
                    if ccost >= min_cost {
                        continue 'movement;
                    }
                }
            }
        }
        if min_cost == usize::max_value() { -1 } else { min_cost as i32 }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_path_1() {
        let maze = vec![
            vec![0,0,1,0,0],
            vec![0,0,0,0,0],
            vec![0,0,0,1,0],
            vec![1,1,0,1,1],
            vec![0,0,0,0,0]
        ];
        assert_eq!(Solution::shortest_distance(maze, vec![0, 4], vec![4, 4]), 12);
    }

    #[test]
    fn test_has_path_2() {
        let maze = vec![
            vec![0,0,1,0,0],
            vec![0,0,0,0,0],
            vec![0,0,0,1,0],
            vec![1,1,0,1,1],
            vec![0,0,0,0,0]
        ];
        assert_eq!(Solution::shortest_distance(maze, vec![0, 4], vec![3, 2]), -1);
    }
}