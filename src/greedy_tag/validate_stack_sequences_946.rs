/*
 * @lc app=leetcode.cn id=946 lang=rust
 *
 * [946] 验证栈序列
 */

// @lc code=start
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let n = pushed.len();
        let mut stack = vec![];
        let mut j = 0;
        for x in pushed {
            stack.push(x);
            while let Some(v) = stack.last() {
                if j < n && *v == popped[j] {
                    stack.pop();
                    j+=1;
                } else {
                    break;
                }
            }
        }
        j == n
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate_stack_sequences() {
        assert!(
            Solution::validate_stack_sequences(
                vec![1,2,3,4,5],
                vec![4,5,3,2,1]
            )
        );
        assert!(
            !Solution::validate_stack_sequences(
                vec![1,2,3,4,5],
                vec![4,3,5,1,2]
            )
        );
    }
}