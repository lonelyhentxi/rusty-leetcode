/*
 * @lc app=leetcode.cn id=656 lang=rust
 *
 * [656] 金币路径
 */

// @star
// @lc code=start
impl Solution {
    pub fn cheapest_jump(a: Vec<i32>, b: i32) -> Vec<i32> {
        let n = a.len();
        if n == 0 || a[n - 1] < 0 {
            return vec![];
        } else if n == 1 {
            return vec![1];
        }
        let mut costs = vec![i64::max_value(); n];
        costs[n - 1] = 0;
        let mut next = vec![-1; n];
        for y in (0..(n - 1)).rev() {
            let x_start = y + 1;
            let x_end = i32::min((n - 1) as i32, (y as i32) + b) as usize;
            let mut min_cost = i64::max_value();
            let mut min_to = -1;
            for x in x_start..=x_end {
                let cost = if a[y] < 0 || costs[x] == i64::max_value() {
                    i64::max_value()
                } else {
                    costs[x] + a[y] as i64
                };
                if cost < min_cost {
                    min_cost = cost;
                    min_to = x as i32;
                }
            }
            costs[y] = min_cost;
            next[y] = min_to;
        }
        if costs[0] == i64::max_value() {
            vec![]
        } else {
            let mut path = vec![0];
            let mut curr = 0i32;
            while curr != (n - 1) as i32 {
                curr = next[curr as usize];
                path.push(curr);
            }
            path.into_iter().map(|v| v + 1).collect::<Vec<_>>()
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coin_path_1() {
        assert_eq!(
            Solution::cheapest_jump(vec![1, 2, 4, -1, 2], 2),
            vec![1, 3, 5]
        );
    }

    #[test]
    fn test_coin_path_2() {
        assert_eq!(Solution::cheapest_jump(vec![1, 2, 4, -1, 2], 1), vec![]);
    }

    #[test]
    fn test_coin_path_3() {
        assert_eq!(Solution::cheapest_jump(vec![0, 0, 0, 0, 0, 0], 3), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_coin_path_4() {
        assert_eq!(Solution::cheapest_jump(vec![0, -1, -1, -1, -1, -1], 5), vec![]);
    }
}
