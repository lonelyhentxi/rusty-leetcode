/*
 * @lc app=leetcode.cn id=255 lang=rust
 *
 * [255] 验证前序遍历序列二叉搜索树
 */
struct Solution;
// @lc code=start

impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        Solution::verify_preorder_rec(&preorder)
    }

    fn verify_preorder_rec(arr: &[i32], ) -> bool {
        if arr.is_empty() {
            return true;
        }
        let root = arr[0];
        let mut i = 1usize;
        while i < arr.len() && arr[i] < root {
            i+=1;
        }
        let left_size = i;
        while i < arr.len() {
            if arr[i] < root {
                return false;
            }
            i+=1;
        }
        Solution::verify_preorder_rec(&arr[1..left_size]) &&
            Solution::verify_preorder_rec(&arr[left_size..arr.len()])
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_preorder_1() {
        let src = vec![5,2,6,1,3];
        assert!(!Solution::verify_preorder(src));
    }

    #[test]
    fn test_verify_preorder_2() {
        let src = vec![5,2,1,3,6];
        assert!(Solution::verify_preorder(src));
    }
}
