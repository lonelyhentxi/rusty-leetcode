/*
 * @lc app=leetcode.cn id=737 lang=rust
 *
 * [737] 句子相似性 II
 */

// @lc code=start
use std::collections::HashMap;

struct UnionFind {
    sz: Vec<usize>,
    id: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            sz: vec![1usize; size],
            id: (0usize..size).collect(),
            size
        }
    }

    pub fn find (&self, mut p: usize) -> usize {
        while self.id[p] != p {
            p = self.id[p];
        }
        p
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let pid = self.find(p);
        let qid = self.find(q);
        if pid == qid {
            return;
        }
        if self.sz[pid] >= self.sz[qid] {
            self.id[qid] = self.id[pid];
            self.sz[pid] += self.sz[qid];
        } else {
            self.id[pid] = self.id[qid];
            self.sz[qid] = self.sz[pid];
        }
        self.size -= 1;
    }
}

impl Solution {
    pub fn are_sentences_similar_two(words1: Vec<String>, words2: Vec<String>, pairs: Vec<Vec<String>>) -> bool {
        if words1.len() != words2.len() {
            return false;
        }
        let mut dict = HashMap::<&str, usize>::new();
        for p in pairs.iter() {
            for i in p {
                let size = dict.len();
                dict.entry(i as &str).or_insert_with(|| size);
            }
        }
        let mut uf = UnionFind::new(dict.len());
        for p in pairs.iter() {
            let pid = dict[&p[0] as &str];
            let qid = dict[&p[1] as &str];
            uf.union(pid, qid);
        }
        let len = words2.len();
        for i in 0..len {
            let p = &words1[i] as &str;
            let q = &words2[i] as &str;
            if p != q {
                let pid_opt = dict.get(p);
                let qid_opt = dict.get(q);
                if let (Some(&pid), Some(&qid)) = (pid_opt, qid_opt) {
                    if !uf.connected(pid, qid) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;
    use crate::{ lc_matrix_s, lc_vec_s };

    #[test]
    fn test_are_sentences_similar_two_1() {
        assert_eq!(
            Solution::are_sentences_similar_two(
                lc_vec_s!["great","acting","skills"],
                lc_vec_s!["fine","drama","talent"],
                lc_matrix_s![["great","good"],["fine","good"],["drama","acting"],["skills","talent"]]
                ),
                true
        );
    }
}