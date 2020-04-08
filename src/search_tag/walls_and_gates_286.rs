/*
 * @lc app=leetcode.cn id=286 lang=rust
 *
 * [286] 墙与门
 */
// @lc code=start
use std::collections::VecDeque;
use std::mem::swap;

const MOVEMENTS: [(isize, isize);4] = [
    (1, 0), (-1,0), (0, 1), (0, -1)
];
const INF: i32 = 2147483647;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        if rooms.is_empty() {
            return;
        }
        if rooms[0].is_empty() {
            return;
        }
        let rows = rooms.len();
        let cols = rooms[0].len();
        for r in 0..rows {
            for c in 0..cols {
                Solution::broadcast_update(rooms, r, c, rows, cols);
            }
        }
    }

    // the value of each grid tells us if it has been visited
    fn broadcast_update(
        rooms: &mut Vec<Vec<i32>>, r: usize, c: usize,
        rows: usize, cols: usize
    ) {
        if rooms[r][c]!=0 {
            return;
        }
        let rows = rows as isize;
        let cols  = cols as isize;
        let mut curr = VecDeque::<(usize, usize)>::new();
        let mut curr_val = 0;
        let mut next = VecDeque::<(usize, usize)>::new();
        curr.push_back((r, c));
        while !curr.is_empty() {
            while let Some((r,c)) = curr.pop_front() {
                let val = rooms[r][c];
                if val==-1 || rooms[r][c] < curr_val {
                    continue;
                }
                rooms[r][c] = curr_val;
                for (dr, dc) in MOVEMENTS.iter() {
                    let nr = dr + (r as isize);
                    let nc = dc + (c as isize);
                    if nr >= 0 && nc>=0 && nr < rows && nc < cols {
                        next.push_back((nr as usize, nc as usize));
                    }
                }
            }
            swap(&mut curr, &mut next);
            curr_val += 1;
        }
    }
}
// @lc code=end
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walls_and_gates() {
        let mut src: Vec<Vec<i32>> = vec![
            vec![INF, -1, 0, INF],
            vec![INF, INF, INF , -1],
            vec![INF, -1, INF, -1], 
            vec![0, -1, INF, INF]
        ];
        let target: Vec<Vec<i32>> = vec![
            vec![3,  -1,   0,   1],
            vec![2,   2,   1,  -1],
            vec![1,  -1,   2,  -1],
            vec![0,  -1,   3,   4],
        ];
        Solution::walls_and_gates(&mut src);
        assert_eq!(src, target);
    }
}