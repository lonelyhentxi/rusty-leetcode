/*
 * @lc app=leetcode.cn id=305 lang=rust
 *
 * [305] 岛屿数量 II
 */

// @lc code=start
struct UnionFind {
    count: usize,
    sz: Vec<usize>,
    id: Vec<usize>,
    land: Vec<bool>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            count: 0,
            sz: vec![1usize; size],
            id: (0usize..size).collect::<Vec<usize>>(),
            land: vec![false; size],
        }
    }

    pub fn add(&mut self, id: usize) {
        if !self.land[id] {
            self.land[id] = true;
            self.count += 1;
        }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p)==self.find(q)
    }

    pub fn find(&self, mut p: usize) -> usize {
        while p!= self.id[p] {
            p = self.id[p]
        }
        p
    }

    pub fn try_union(&mut self, p: usize, q: usize) {
        if self.land[p] && self.land[q] {
            let pid = self.find(p);
            let qid = self.find(q);
            if pid==qid {
                return;
            }
            if self.sz[pid]>self.sz[qid] {
                self.id[qid] = pid;
                self.sz[pid] += self.sz[qid];
            }
            else {
                self.id[pid] = qid;
                self.sz[qid] += self.sz[pid];
            }
            self.count -= 1;
        }
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
}

const MOVEMENTS: [(isize, isize);4] = [(1,0), (-1,0), (0,1), (0,-1)];

impl Solution {
    #[inline(always)]
    fn pos2id(x: usize, y: usize, _rows: usize, cols: usize) -> usize {
        x * cols + y
    }

    pub fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        let m = m as usize;
        let n = n as usize;
        let points = (m*n) as usize;
        let mut counts = vec![];
        let mut uf = UnionFind::new(points);
        for pos in positions {
            let x = pos[0] as usize;
            let y = pos[1] as usize;
            let id = Solution::pos2id(x,y,m,n);
            uf.add(id);
            for movement in &MOVEMENTS {
                let next_x = x as isize + movement.0;
                let next_y = y as isize + movement.1;
                if next_x>=0 && next_y>=0 && next_x < (m as isize) && next_y < (n as isize) {
                    let next_id = Solution::pos2id(next_x as usize, next_y as usize, m, n);
                    uf.try_union(id, next_id);
                }
            }
            counts.push(uf.count() as i32);
        }
        counts
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_islands2() {
        let positions = [[0,0], [0,1], [1,2], [2,1]].iter().map(|x| x.to_vec()).collect::<Vec<Vec<i32>>>();
        let expected = vec![1,1,2,3];
        assert_eq!(Solution::num_islands2(3,3,positions), expected);
    }
}