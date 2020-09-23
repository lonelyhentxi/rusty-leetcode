#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}
/*
 * @lc app=leetcode.cn id=364 lang=rust
 *
 * [364] 嵌套列表权重和 ii
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
  pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
    let root = &NestedInteger::List(nested_list);
    let mut queue: VecDeque<(&NestedInteger, i32)> = VecDeque::new();
    queue.push_back((root, 0));
    let mut max_height = 0;
    while let Some((node, level)) = queue.pop_back() {
      match node {
        &NestedInteger::List(ref v) => {
          for i in v {
            queue.push_back((i, level+1));
          }
        },
        _ => max_height = i32::max(max_height, level)
      }
    }
    queue.clear();
    queue.push_back((root, 0));
    let mut sum = 0;
    while let Some((node, level)) = queue.pop_front() {
      match node {
        &NestedInteger::List(ref v) => {
          for i in v {
            queue.push_back((i, level+1));
          }
        },
        &NestedInteger::Int(v) => {
          sum += v * (max_height - level + 1);
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
    use super::NestedInteger::*;

    #[test]
    fn test_depth_sum_inverse_1() {
      assert_eq!(Solution::depth_sum_inverse(
        vec![List(vec![Int(1),  Int(1)]), Int(2), List(vec![Int(1), Int(1)])]
      ), 8);
    }

    #[test]
    fn test_depth_sum_inverse_2() {
      assert_eq!(Solution::depth_sum_inverse(
        vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])]
      ), 17);
    }
}