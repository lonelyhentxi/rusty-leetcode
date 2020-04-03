/*
 * @lc app=leetcode.cn id=256 lang=rust
 *
 * [256] 粉刷房子
 */
struct Solution;
// @lc code=start

const COLOR_SIZE: usize = 3;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp: [[i32; COLOR_SIZE];2] = [[0, 0, 0],[0, 0, 0]];
        let mut iter = 0;
        for c in costs {
            let another_iter = iter;
            iter = (iter+1) % 2;
            for i in 0..COLOR_SIZE {
                dp[iter][i]
                    = i32::min(
                    dp[another_iter][(i+1)%COLOR_SIZE],
                    dp[another_iter][(i+2)%COLOR_SIZE])
                    + c[i];
            }
        }
        dp[iter].iter()
            .fold(i32::max_value(),
                  |sum, curr| i32::min(sum, *curr))
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost() {
        let inputs = vec![vec![17,2,17],vec![16,16,5],vec![14,3,19]];
        assert_eq!(Solution::min_cost(inputs), 10);
    }
}
