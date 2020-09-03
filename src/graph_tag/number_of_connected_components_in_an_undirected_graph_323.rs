/*
 * @lc app=leetcode.cn id=323 lang=rust
 *
 * [323] 无向图中连通分量的数目
 */

// @lc code=start

struct UnionFind {
    sz: Vec<usize>,
    id: Vec<usize>,
    count: usize
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            sz: vec![0; size],
            id: (0usize..size as usize).collect(),
            count: size
        }
    }

    pub fn find(&self, mut p: usize) -> usize {
        let id = &self.id;
        loop {
            let q = id[p];
            if q == p {
                return q;
            } else {
                p = q;
            }
        }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.find(p);
        let qid = self.find(q);
        if pid != qid {
            if self.sz[pid] > self.sz[qid] {
                self.id[qid] = pid;
                self.sz[pid] += self.sz[qid];
            } else {
                self.id[pid] = qid;
                self.sz[qid] += self.sz[pid];
            }
            self.count -= 1;
        }
    }
}

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        for e in edges {
            let p = e[0] as usize;
            let q = e[1] as usize;
            uf.union(p, q);
        }
        uf.count as i32
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_components_1() {
        assert_eq!(Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]]), 2);
    }

    
    #[test]
    fn test_count_components_2() {
        assert_eq!(Solution::count_components(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]]), 1);
    }
}