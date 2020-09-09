/*
 * @lc app=leetcode.cn id=325 lang=rust
 *
 * [325] 和等于 k 的最长子数组长度
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
        let from_to_here_sum: Vec<i32> = (0..=nums.len())
            .into_iter()
            .scan(0, |acc, curr| {
                *acc += if curr == 0 { 0 } else { nums[curr-1] };
                Some(*acc)
            })
            .collect();
        let first_sum_to_here_index = from_to_here_sum
            .iter()
            .enumerate()
            .rev()
            .map(|(i, sum)| (*sum, i))
            .collect::<HashMap<i32, usize>>();
        from_to_here_sum
            .iter()
            .enumerate()
            .skip(1)
            .map(|(to, to_sum)| {
                let from_sum = to_sum - k;
                let map = &first_sum_to_here_index;
                if map.contains_key(&from_sum) && to > map[&from_sum] {
                    Some(to - map[&from_sum])
                } else {
                    None
                }
            }).fold(0usize, |acc, curr| {
                usize::max(acc, match curr {
                    Some(len) => len,
                    None => 0
                })
            }) as i32

    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_sub_array_len_1() {
        assert_eq!(Solution::max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
    }

    #[test]
    fn test_max_sub_array_len_2() {
        assert_eq!(Solution::max_sub_array_len(vec![-2, -1, 2, 1], 1), 2);
    }
}