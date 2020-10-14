/*
 * @lc app=leetcode.cn id=499 lang=rust
 *
 * [499] 迷宫 III
 */

// @lc code=start
use std::collections::{ HashMap, VecDeque };

type Pos = (isize, isize);
const MOVEMENTS: &[(isize, isize, char); 4] = &[(-1, 0, 'u'), (0, 1, 'r'), (1, 0, 'd'), (0, -1 ,'l')];

impl Solution {
    pub fn find_shortest_way(maze: Vec<Vec<i32>>, ball: Vec<i32>, hole: Vec<i32>) -> String {
        let impossible = String::from("impossible");
        let rows = maze.len() as isize;
        let cols = if rows == 0 { 0 } else { maze[0].len() as isize };
        if rows == 0 || cols == 0 { return impossible.clone(); };
        let start = (ball[0] as isize, ball[1] as isize);
        let dest = (hole[0] as isize, hole[1] as isize);
        {
            let (y, x) = start;
            if x < 0 || y < 0 || y >= rows || x >= cols || maze[y as usize][x as usize] != 0 {
                return impossible.clone();
            }
        }
        if start == dest { return String::new(); };
        let mut queue = VecDeque::<Pos>::new();
        let mut memo = HashMap::<Pos, (usize, String)>::new();
        queue.push_back(start);
        memo.insert(start, (0, String::new()));
        let mut min_cost = (usize::max_value(), String::from("z"));
        while let Some(pos) = queue.pop_front() {
            let (y, x) = pos;
            'movement: for &(dy, dx, d) in MOVEMENTS {
                let mut cy = y;
                let mut cx = x;
                let mut ccost = memo[&pos].clone();
                ccost.1.push(d);
                loop {
                    let ny = cy + dy;
                    let nx = cx + dx;
                    let cpos = (cy, cx);
                    let npos = (ny, nx);
                    if npos == dest {
                        min_cost = if ccost < min_cost { ccost.clone() } else { min_cost };
                        continue 'movement;
                    }
                    let next_invalid = ny < 0 || nx < 0 || ny >= rows || nx >= cols || maze[ny as usize][nx as usize] != 0;
                    if next_invalid {
                        if !memo.contains_key(&cpos) || ccost < memo[&cpos] {
                            queue.push_back(cpos);
                            memo.insert(cpos, ccost);
                        }
                        continue 'movement;
                    }
                    cy = ny;
                    cx = nx;
                    ccost.0 += 1;
                    if min_cost < ccost {
                        continue 'movement;
                    }
                }
            }
        }
        if min_cost.0 == usize::max_value() { impossible } else { min_cost.1 }
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
            vec![0,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,0,0],
            vec![0,1,0,0,1],
            vec![0,1,0,0,0]
        ];
        assert_eq!(Solution::find_shortest_way(maze, vec![4, 3], vec![0, 1]), String::from("lul"));
    }

    #[test]
    fn test_has_path_2() {
        let maze = vec![
            vec![0,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,0,0],
            vec![0,1,0,0,1],
            vec![0,1,0,0,0]
        ];
        assert_eq!(Solution::find_shortest_way(maze, vec![4, 3], vec![3, 0]), String::from("impossible"));
    }

    #[test]
    fn test_has_path_3() {
        let maze = vec![
            vec![0,0,0,0,0,0,0],
            vec![0,0,1,0,0,1,0],
            vec![0,0,0,0,1,0,0],
            vec![0,0,0,0,0,0,1]
        ];
        assert_eq!(Solution::find_shortest_way(maze, vec![0, 4], vec![3, 5]), String::from("dldr"));
    }
}
