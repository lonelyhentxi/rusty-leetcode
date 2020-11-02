/*
 * @lc app=leetcode.cn id=582 lang=rust
 *
 * [582] 杀死进程
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn kill_process(pid: Vec<i32>, ppid: Vec<i32>, kill: i32) -> Vec<i32> {
        let len = pid.len();
        if len == 0 {
            return vec![];
        }
        let mut ps = HashMap::<usize, Vec<usize>>::new();
        for i in 0..len {
            let cpid = pid[i] as usize;
            let ccpid = ppid[i] as usize;
            ps.entry(ccpid)
                .and_modify(|v| v.push(cpid))
                .or_insert_with(|| vec![cpid]);
        }
        let mut res = vec![];
        let mut stack = vec![];
        stack.push(kill as usize);
        while let Some(n) = stack.pop() {
            res.push(n as i32);
            if ps.contains_key(&n) {
                stack.extend(ps[&n].iter());
            }
        }
        res
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kill_process_1() {
        assert_eq!(Solution::kill_process(vec![1, 3, 10, 5], vec![3, 0, 5, 3], 5), [5, 10]);
    }
}