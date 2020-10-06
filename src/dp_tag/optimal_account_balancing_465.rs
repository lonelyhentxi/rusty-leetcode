/*
 * @lc app=leetcode.cn id=465 lang=rust
 *
 * [465] 最优账单平衡
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut balance = HashMap::<i32, i32>::new();
        for v in transactions {
            let i = v[0];
            let o = v[1];
            let a = v[2];
            balance.entry(i)
                .and_modify(|b| *b += a)
                .or_insert(a);
            balance.entry(o)
                .and_modify(|b| *b -= a)
                .or_insert(0 - a);
        }
        let employees = balance
            .into_iter()
            .filter(|(_, account)| *account != 0)
            .map(|(_, account)| account)
            .collect::<Vec<i32>>();
        let len = employees.len() as i32;
        let size = 1usize << len;
        let mut sum = vec![0; size];
        for i in 0..(1 << len) {
            for j in 0..len {
                if i & (1 << j) != 0 {
                    sum[i] = sum[i ^ (1 << j)] + employees[j as usize];
                    break;
                }
            }
        }
        let mut dp = vec![0; size];
        for i in 1..(size as i32) {
            if sum[i as usize] != 0 {
                continue;
            }
            dp[i as usize] = 1;
            let mut si = (i - 1) & i;
            while si != 0 {
                if dp[si as usize] != 0 {
                    dp[i as usize] = i32::max(dp[i as usize], dp[si as usize] + 1);
                }
                si = (si - 1) & i;
            }
        }
        (len as i32)  -  dp[size - 1]
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_transfers_1() {
        assert_eq!(Solution::min_transfers(vec![vec![0, 1, 10], vec![2, 0, 5]]), 2);
    }

    #[test]
    fn test_min_transfers_2() {
        assert_eq!(Solution::min_transfers(vec![
            vec![0,1,10], 
            vec![1,0,1], 
            vec![1,2,5], 
            vec![2,0,5]]), 1);
    }
}
