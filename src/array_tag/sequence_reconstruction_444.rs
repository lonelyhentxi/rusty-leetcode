/*
 * @lc app=leetcode.cn id=444 lang=rust
 *
 * [444] 序列重建
 */

// @lc code=start
use std::collections::{ HashMap, HashSet };

impl Solution {
    pub fn sequence_reconstruction(org: Vec<i32>, seqs: Vec<Vec<i32>>) -> bool {
        let mut org_set = org.iter().map(|k| (*k, 0usize)).collect::<HashMap<i32, usize>>();
        let mut ins: HashMap<i32, usize> = HashMap::new();
        let mut outs: HashMap<i32, HashSet<i32>> = HashMap::new();
        for s in &seqs {
            for i in s {
                if !org_set.contains_key(i) {
                    return false;
                } else {
                    *org_set.get_mut(i).unwrap() += 1;
                }
            }
            for j in 1..s.len() {
                let i = s[j - 1];
                let o = s[j];
                let mut is_dup = false;
                outs.entry(o)
                    .and_modify(|v| {
                        if !v.contains(&i) {
                            v.insert(i);
                        } else {
                            is_dup = true
                        }
                    })
                    .or_insert({ let mut s = HashSet::new(); s.insert(i); s });
                ins.entry(i)
                    .and_modify(|v| { 
                        if !is_dup {
                            *v = *v + 1;
                        }
                     })
                    .or_insert(1);
                ins.entry(o).or_insert(0);
            }
        }
        for (_, v) in org_set {
            if v == 0 {
                return false;
            }
        }
        for j in 1..org.len() {
            let i = org[j - 1];
            let o = org[j];
            if !outs.contains_key(&o) || !outs[&o].contains(&i) {
                return false;
            }
        }
        while !ins.is_empty() {
            let zeros_ins: HashSet<i32> = ins.iter().filter(|(_, c)| **c == 0).map(|(i, _)| *i).collect::<HashSet<i32>>();
            // println!("{:?}", zeros_ins);
            // println!("{:?}, {:?}", ins, outs);
            if zeros_ins.is_empty() && !seqs.is_empty() {
                return false;
            }
            for i in zeros_ins {
                if outs.contains_key(&i) {
                    for o in &outs[&i] {
                        let v = ins.get_mut(&o).unwrap();
                        *v -= 1;
                    }
                    outs.remove(&i);
                }
                ins.remove(&i);
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

    #[test]
    fn test_sequence_reconstruction_1() {
        assert!(!Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1,2],vec![1,3]]));
    }

    #[test]
    fn test_sequence_reconstruction_2() {
        assert!(!Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 3]]));
    }

    #[test]
    fn test_sequence_reconstruction_3() {
        assert!(!Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 2]]));
    }

    #[test]
    fn test_sequence_reconstruction_4() {
        assert!(Solution::sequence_reconstruction(vec![1, 2, 3], vec![vec![1, 2], vec![1, 3], vec![2, 3]]));
    }

    #[test]
    fn test_sequence_reconstruction_5() {
        assert!(Solution::sequence_reconstruction(vec![4,1,5,2,6,3], vec![vec![5, 2, 6, 3], vec![4, 1, 5, 2]]));
    }
}