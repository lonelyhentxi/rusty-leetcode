/*
 * @lc app=leetcode.cn id=711 lang=rust
 *
 * [711] 不同岛屿的数量 II
 */

// @lc code=start
use std::collections::{ HashSet, HashMap };

const TRANS_MATS: &[[[i32;2];2];8] = &[
    [[1, 0], [0, 1]],
    [[1, 0], [0, -1]],
    [[-1, 0], [0, 1]],
    [[-1, 0], [0, -1]],
    [[0, 1], [-1, 0]],
    [[0, 1], [1, 0]],
    [[0, -1], [1, 0]],
    [[0, -1], [-1, 0]]
];

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
    pub fn num_distinct_islands2(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = if rows == 0 { 0 } else { grid[0].len() };
        let size = rows * cols;
        if size == 0 {
            return 0;
        }
        let mut uf = UnionFind::new(size);
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
        let mut islands = HashMap::<usize, Vec<(usize, usize)>>::new();
        for i in 0..size {
            let x = i % cols;
            let y = i / cols;
            if grid[y][x] == 1 {
                let id = uf.find(i);
                islands.entry(id)
                    .and_modify(|v| v.push((x,y)))
                    .or_insert_with(|| vec![(x,y)]);
            }
        }
        let mut unique = HashSet::<Vec<i32>>::new();
        'outer: for (_, points) in &islands {
            let mut ident = vec![];
            for tm in TRANS_MATS {
                let mut trans_points = points.iter().map(|source|  Solution::trans(source.0 as i32, source.1 as i32, tm)).collect::<Vec<_>>();
                trans_points.sort();
                let min = trans_points[0];
                ident = trans_points.into_iter().map(|(x, y)|  (y - min.1) * (cols as i32) +(x - min.0)).collect();
                if unique.contains(&ident) {
                    continue 'outer;
                }
            }
            unique.insert(ident);
        }
        unique.len() as i32
    }

    #[inline]
    fn trans(cx: i32, cy: i32, trans_mat: &[[i32;2];2]) -> (i32, i32) {
        (cx * trans_mat[0][0] + cy * trans_mat[1][0], cx * trans_mat[0][1] + cy * trans_mat[1][1])
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_num_distinct_islands2_1() {
        let grid = lc_matrix![[1,1,0,0,0],[1,0,0,0,0],[0,0,0,0,1],[0,0,0,1,1]];
        assert_eq!(Solution::num_distinct_islands2(grid), 1);
    }

    #[test]
    fn test_num_distinct_islands2_2() {
        let grid = lc_matrix![[1,1,1,0,0],[1,0,0,0,1],[0,1,0,0,1],[0,1,1,1,0]];
        assert_eq!(Solution::num_distinct_islands2(grid), 2);
    }
}