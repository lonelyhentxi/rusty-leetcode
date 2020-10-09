/*
 * @lc app=leetcode.cn id=484 lang=rust
 *
 * [484] 寻找排列
 */

// @lc code=start
impl Solution {
    pub fn find_permutation(s: String) -> Vec<i32> {
        let chars = s.chars().collect::<Vec<char>>();
        let len = chars.len();
        let mut res = (1..=(len + 1) as i32).into_iter().collect::<Vec<i32>>();
        let mut i = 1usize;
        while i <= len {
            let j = i;
            while i <= len && chars[i - 1] == 'D' {
                i += 1;
            }
            Solution::reverse(&mut res[(j - 1)..i]);
            i += 1;
        }
        res
    }

    fn reverse(arr: &mut [i32]) {
        let len = arr.len();
        let mid = len / 2;
        for i in 0..mid {
            arr.swap(i, len - 1 - i);
        }
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_permutation_1() {
        assert_eq!(Solution::find_permutation(String::from("I")), vec![1,2]);
    }

    #[test]
    fn test_find_permutation_2() {
        assert_eq!(Solution::find_permutation(String::from("DI")), vec![2,1,3]);
    }
}

