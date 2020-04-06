/*
 * @lc app=leetcode.cn id=276 lang=rust
 *
 * [276]  栅栏涂色
 */
 // @lc code=start
 impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        // per color
        if n==0 || k==0 {
            return 0;
        }
        let mut prev_same = 0;
        let mut prev_diff = 1;
        for _ in 1..n {
            let diff = 
                (k-1) * (prev_diff + prev_same);
            let same = prev_diff;
            println!("{},{}", same, diff);
            prev_same = same;
            prev_diff = diff;
        }
        (prev_same + prev_diff) * k
    }
}
 // @lc code=end
 struct Solution;
 
 #[cfg(test)]
 mod test {
     use super::*;
     #[test]
     fn test_num_ways() {
         assert_eq!(Solution::num_ways(3, 3), 24);
     }
 }
 