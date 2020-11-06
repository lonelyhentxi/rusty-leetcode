/*
 * @lc app=leetcode.cn id=624 lang=rust
 *
 * [624] 数组列表中的最大距离
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let len = arrays.len();
        if len <= 1 {
            return 0;
        }
        let mut mins = BinaryHeap::<(i32, usize)>::new();
        let mut maxs = BinaryHeap::<Reverse<(i32, usize)>>::new();
        for i in 0..len {
            let a = &arrays[i];
            let min_add = if mins.len() < 2 {
                (a[0], i)
            } else {
                let last_min_2 = mins.pop().unwrap();
                if a[0] < last_min_2.0 {
                    (a[0], i)
                } else {
                    last_min_2
                }
            };
            mins.push(min_add);
            let max_add = Reverse(if maxs.len() < 2 {
                (a[a.len() - 1], i)
            } else {
                let Reverse(last_max_2) = maxs.pop().unwrap();
                if a[a.len() - 1] > last_max_2.0 {
                    (a[a.len() - 1], i)
                } else {
                    last_max_2
                }
            });
            maxs.push(max_add);
        }
        let maxs = maxs.into_iter().map(|Reverse(v)| v).collect::<Vec<_>>();
        let mins = mins.into_iter().collect::<Vec<_>>();
        if maxs[1].1 != mins[1].1 {
            i32::abs(maxs[1].0 - mins[1].0)
        } else {
            i32::max(
                i32::abs(maxs[0].0 - mins[1].0),
                i32::abs(maxs[1].0 - mins[0].0),
            )
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_matrix;

    #[test]
    fn test_max_distance_1() {
        assert_eq!(
            Solution::max_distance(lc_matrix![[1, 2, 3], [0, 4, 5], [1, 2, 3]]),
            4
        );
    }
}
