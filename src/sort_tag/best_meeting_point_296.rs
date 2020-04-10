/*
 * @lc app=leetcode.cn id=296 lang=rust
 *
 * [296] 最佳的碰头地点
 */

// @lc code=start
impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        if grid[0].is_empty() {
            return 0;
        }
        let mut rows = vec![];
        for (i, r) in grid.iter().enumerate() {
            for v in r.iter() {
                if *v==1 {
                    rows.push(i);
                }
            }
        }
        let mut cols = vec![];
        for j in 0..grid[0].len() {
            for i in 0..grid.len() {
                if grid[i][j]==1 {
                    cols.push(j);
                }
            }
        }
        (Solution::min_distance_linear(&rows) 
        + Solution::min_distance_linear(&cols)) as i32
    }

    fn min_distance_linear(arr: &[usize]) -> usize {
        if arr.is_empty() {
            return 0;
        }
        let mut distance = 0usize;
        let mut i = 0usize;
        let mut j = arr.len() - 1;
        while i<j {
            distance += (arr[j] - arr[i]) as usize;
            i+=1;
            j-=1;
        }
        return distance;
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test  {
    use super::*;

    #[test]
    fn test_min_total_distance() {
        let matrix = vec![
            vec![1,0,0,0,1],
            vec![0,0,0,0,0],
            vec![0,0,1,0,0]
        ];
        assert_eq!(Solution::min_total_distance(matrix), 6);
    }
}

