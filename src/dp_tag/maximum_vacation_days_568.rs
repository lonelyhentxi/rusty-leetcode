/*
 * @lc app=leetcode.cn id=568 lang=rust
 *
 * [568] 最大休假天数
 */

// @lc code=start
impl Solution {
    pub fn max_vacation_days(flights: Vec<Vec<i32>>, days: Vec<Vec<i32>>) -> i32 {
        let clen = days.len();
        let dlen = if clen == 0 { 0 } else { days[0].len() };
        if dlen + clen == 0 {
            return 0;
        }
        let mut dict = vec![vec![]; clen as usize];
        for j in 0..clen {
            dict[j].push(j);
            for i in 0..clen {
                if flights[i][j] == 1 {
                    dict[j].push(i);
                }
            }
        }
        let mut dp = vec![vec![i32::min_value(); clen]; 2];
        dp[1][0] = 0;
        for i in 0..dlen {
            let mod_i = i % 2;
            let mod_prev_i = (i + 1) % 2;
            for j in 0..clen {
                let mut j_v = i32::min_value();
                for &k in &dict[j] {
                    j_v = i32::max(dp[mod_prev_i][k] + days[j][i], j_v);
                }
                dp[mod_i][j] = j_v;
            }
        }
        let last_mod_i = (dlen - 1) % 2;
        dp[last_mod_i].iter().cloned().max().unwrap()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_vacation_days_1() {
        let flights = vec![
            vec![0,1,1],vec![1,0,1],vec![1,1,0]
        ];
        let days = vec![
            vec![1,3,1], vec![6, 0, 3], vec![3, 3, 3]
        ];
        assert_eq!(Solution::max_vacation_days(flights, days), 12);
    }

    #[test]
    fn test_max_vacation_days_2() {
        let flights = vec![
            vec![0,0,0],vec![0,0,0],vec![0,0,0]
        ];
        let days = vec![
            vec![1,1,1], vec![7,7,7], vec![7,7,7]
        ];
        assert_eq!(Solution::max_vacation_days(flights, days), 3);
    }

    #[test]
    fn test_max_vacation_days_3() {
        let flights = vec![
            vec![0,1,1],vec![1,0,1],vec![1,1,0]
        ];
        let days = vec![
            vec![7,0,0], vec![0,7,0], vec![0,0,7]
        ];
        assert_eq!(Solution::max_vacation_days(flights, days), 21);
    }
}