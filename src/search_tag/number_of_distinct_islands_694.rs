/*
 * @lc app=leetcode.cn id=694 lang=rust
 *
 * [694] 不同岛屿的数量
 */

// @lc code=start
use std::collections::{ HashSet, HashMap };

struct UnionFind {
    sz: Vec<usize>,
    id: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            sz: vec![0; size],
            id: (0..size).collect(),
            count: size
        }
    }

    pub fn find(&self, mut p: usize) -> usize {
        while self.id[p] != p {
            p = self.id[p];
        }
        p
    }

    pub fn connected(&self, p:usize, q:usize) -> bool {
        let pid = self.find(p);
        let qid = self.find(q);
        pid == qid
    }

    pub fn union(&mut self, p:usize, q:usize) {
        let pid = self.find(p);
        let qid = self.find(q);
        if pid == qid {
            return;
        }
        if self.sz[pid] > self.sz[qid] {
            self.id[qid] = self.id[pid];
        } else {
            self.id[pid] = self.id[qid];
        }
    }
}

impl Solution {
    pub fn num_distinct_islands(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };
        if rows * cols == 0 {
            return 0;
        }
        let mut uf = UnionFind::new(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    let k = i * cols + j;
                    if i > 0 && grid[i-1][j] == 1 {
                        let k_up = k - cols;
                        uf.union(k, k_up);
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        let k_left = k - 1;
                        uf.union(k, k_left);
                    }
                }
            }
        }
        let mut islands = HashMap::<usize, Vec<i32>>::new();
        for i in 0..rows * cols {
            let r = i / cols;
            let c =  i % cols;
            if grid[r][c] == 1 {
                let id = uf.find(i);
                let relative = (i as i32) - (id as i32);
                islands.entry(id)
                    .and_modify(|v| v.push(relative))
                    .or_insert_with(|| vec![relative]);
            }
        }
        islands.into_iter().map(|(_, v)| v).collect::<HashSet<_>>().len() as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_num_distinct_islands_1() {
        let grid = lc_matrix![
            [1,1,0,0,0],
            [1,1,0,0,0],
            [0,0,0,1,1],
            [0,0,0,1,1]
        ];
        assert_eq!(Solution::num_distinct_islands(grid), 1);
    }

    #[test]
    fn test_num_distinct_islands_2() {
        let grid = lc_matrix![
            [1,1,0,1,1],
            [1,0,0,0,0],
            [0,0,0,0,1],
            [1,1,0,1,1]
        ];
        assert_eq!(Solution::num_distinct_islands(grid), 3);
    }
}