/*
 * @lc app=leetcode.cn id=490 lang=rust
 *
 * [490] 迷宫
 */

// @lc code=start
use std::collections::VecDeque;
use std::collections::HashSet;

type Pos = (isize, isize);
type Dir = usize;

const MOVEMENTS: &[Pos; 4] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

impl Solution {
    pub fn has_path(maze: Vec<Vec<i32>>, start: Vec<i32>, destination: Vec<i32>) -> bool {
        let rows = maze.len();
        let cols = if rows == 0 { 0 } else { maze[0].len() };
        if rows == 0 || cols == 0 { return false };
        let mut queue = VecDeque::<(isize, isize)>::new();
        let mut set = HashSet::new();
        let start = (start[0] as isize, start[1] as isize);
        let dest = (destination[0] as isize, destination[1] as isize);
        let start_valid = start.0 >= 0 && start.1 >= 0 && start.0 < (rows as isize) && start.1 < (cols as isize) && maze[start.0 as usize][start.1 as usize] == 0;
        if !start_valid {
            return false;
        }
        if start == dest {
            return true;
        }
        queue.push_back(start);
        set.insert(start);
        while let Some((y, x)) = queue.pop_front() {
            'outer: for &(dy, dx) in MOVEMENTS {
                println!("x: {:?}, y: {:?}, dx: {:?}, dy: {:?}", x, y, dx, dy);
                let mut cx = x;
                let mut cy = y;
                loop {
                    let nx = cx + dx;
                    let ny = cy + dy;
                    let next_valid = nx >= 0 && ny >= 0 && nx < (cols as isize) && ny < (rows as isize) && maze[ny as usize][nx as usize] == 0;
                    let curr_pos = (cy, cx);
                    if !next_valid {
                        if !(cx == x && cy == y) && !set.contains(&curr_pos) {
                            if curr_pos == dest {
                                return true;
                            }
                            queue.push_back(curr_pos);
                            set.insert(curr_pos);
                        }
                        continue 'outer;
                    }
                    cx = nx;
                    cy = ny;
                }
            }
        }
        false
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
        assert_eq!(Solution::has_path(maze, vec![0, 4], vec![4, 4]), true);
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
        assert_eq!(Solution::has_path(maze, vec![0, 4], vec![3, 2]), false);
    }
}
