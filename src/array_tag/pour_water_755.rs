/*
 * @lc app=leetcode.cn id=755 lang=rust
 *
 * [755] 倒水
 */

// @lc code=start
impl Solution {
    pub fn pour_water(mut heights: Vec<i32>, v: i32, k: i32) -> Vec<i32> {
        let len = heights.len() as i32;
        let mut v = v;
        while v > 0 {
            let mut i = k - 1;
            let mut left_low = k;
            while i >= -1 {
                let h = if i >= 0 { heights[i as usize] } else { i32::max_value() };
                let prev_i = (i + 1) as usize;
                let prev_h = heights[prev_i];
                if h > prev_h {
                    break;
                } else if h < prev_h {
                    left_low = i;
                }
                i -= 1;
            }
            if left_low == k {
                let mut i = k + 1;
                let mut right_low = k;
                while i <= len {
                    let h = if i < len { heights[i as usize] } else { i32::max_value() };
                    let prev_i = i as usize - 1;
                    let prev_h = heights[prev_i];
                    if h > prev_h {
                        break;
                    } else if h < prev_h {
                        right_low = i;
                    }
                    i += 1;
                }
                heights[right_low as usize] += 1;
            } else {
                heights[left_low as usize] += 1;
            }
            v -= 1;
        }
        heights
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_pour_water_1() {
        assert_eq!(Solution::pour_water(vec![2,1,1,2,1,2,2], 4, 3), vec![2,2,2,3,2,2,2])
    }


    #[test]
    fn test_pour_water_2() {
        assert_eq!(Solution::pour_water(vec![1,2,3,4], 2, 2), vec![2,3,3,4]);
    }

    #[test]
    fn test_pour_water_3() {
        assert_eq!(Solution::pour_water(vec![3, 1, 3], 5, 1), [4, 4, 4]);
    }
}