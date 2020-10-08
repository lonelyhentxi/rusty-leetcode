/*
 * @lc app=leetcode.cn id=469 lang=rust
 *
 * [469] 凸多边形
 */

// @lc code=start
impl Solution {
    pub fn is_convex(points: Vec<Vec<i32>>) -> bool {
        let points = points.into_iter().map(|v| (v[0], v[1])).collect::<Vec<_>>();
        let len = points.len();
        if len <= 3 {
            return true;
        }
        let mut direction: i64 = 0;
        for i in 0..len {
          let (x1, y1) = points[i % len];
          let (x2, y2) = points[(i + 1) % len];
          let (x3, y3) = points[(i + 2) % len];
          let new_directon = Solution::cross((x2 - x1, y2 - y1), (x3 - x1, y3 - y1));
          if new_directon != 0 {
            if new_directon * direction < 0 {
              return false;
            }
            direction = new_directon;
          }
        }
        return true;
    }

    pub fn cross(p1: (i32, i32), p2: (i32, i32)) -> i64 {
        (p1.0 as i64) * (p2.1 as i64) - (p2.0 as i64) * (p1.1 as i64)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_convex_1() {
    assert!(Solution::is_convex(vec![vec![0,0],vec![0,1],vec![1,1],vec![1,0]]));
  }

  #[test]
  fn test_is_convex_2() {
    assert!(!Solution::is_convex(vec![vec![0,0],vec![0,10],vec![10,10],vec![10,0], vec![5, 5]]));
  }
}
