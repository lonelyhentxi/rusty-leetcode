/*
 * @lc app=leetcode.cn id=261 lang=rust
 *
 * [261] 以图判树
 */

// @lc code=start
struct UnionFind {
    sz: Vec<usize>,
    id: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            id:  (vec![0;size]).iter().enumerate().map(|(i,_)| i).collect(),
            sz: vec![1; size],
            count: size,
        }
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.find(p);
        let qid = self.find(q);
        if pid==qid {
            return;
        }
        if self.sz[pid] > self.sz[qid] {
            self.id[qid] = pid;
            self.sz[pid] += self.sz[qid];
        } else {
            self.id[pid] = qid;
            self.sz[qid] = self.sz[pid];
        }
        self.count -= 1;
    }

    pub fn find(&self, mut p: usize) -> usize {
        loop {
            let pid = self.id[p];
            if pid==p {
                return p;
            }
            p = pid;
        }
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }
}

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        if n < 0 {
            return false;
        } else if n ==0 {
            return true;
        }
        let mut uf = UnionFind::new(n as usize);
        for edge in edges {
            let s = edge[0] as usize;
            let e = edge[1] as usize;
            if uf.connected(s, e) {
                return false;
            }
            uf.union(s, e);
        }
        uf.count==1
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_tree_1() {
        let edges = vec![vec![0,1], vec![0,2], vec![0,3], vec![1,4]];
        assert!(Solution::valid_tree(5, edges));
    }

    #[test]
    fn test_valid_tree_2() {
        let edges = vec![vec![0,1], vec![1,2], vec![2,3], vec![1,3], vec![1,4]];
        assert!(!Solution::valid_tree(5, edges));
    }
}