#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

/*
 * @lc app=leetcode.cn id=339 lang=rust
 *
 * [339] 嵌套列表权重和
 */
 // @lc code=start
use std::collections::VecDeque;


impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        let mut deque = VecDeque::<(NestedInteger, i32)>::new();
        deque.push_back((NestedInteger::List(nested_list), 0));
        let mut sum = 0;
        while let Some((ni, level)) = deque.pop_front() {
            match ni {
                NestedInteger::Int(num) => sum += num * level,
                NestedInteger::List(list) => {
                    list.into_iter().for_each(|i| { deque.push_back((i, level + 1)) });
                }
            }
        }
        sum
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use NestedInteger::{List, Int};

    #[test]
    fn test_depth_sum_1() {
        assert_eq!(Solution::depth_sum(
            vec![List(vec![Int(1), Int(1)]), Int(2), List(vec![Int(1),  Int(1)])]
        ), 10);
    }

    #[test]
    fn test_depth_sum_2() {
        assert_eq!(Solution::depth_sum(
            vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])]
        ), 27);
    }

    
    #[test]
    fn test_depth_sum_3() {
        assert_eq!(Solution::depth_sum(
            vec![Int(0)]
        ), 0);
    }

    #[test]
    fn test_depth_sum_4() {
        assert_eq!(Solution::depth_sum(
            vec![]
        ), 0);
    }
}