/*
 * @lc app=leetcode.cn id=265 lang=rust
 *
 * [265] 粉刷房子 II
 */

// @lc code=start

struct CostRecord {
    min_cost: i32,
    second_min_cost: i32,
    min_id: usize,
}

impl CostRecord {
    pub fn new(init_cost: i32) -> Self {
        Self {
            min_cost: init_cost,
            second_min_cost: init_cost,
            min_id: usize::max_value()
        }
    }

    pub fn add(&mut self, last_record: &CostRecord, id: usize, cost: i32) {
        let new_all_cost = (if id==last_record.min_id {
            last_record.second_min_cost
        } else {
            last_record.min_cost
        }) + cost;
        if new_all_cost < self.min_cost {
            self.second_min_cost = self.min_cost;
            self.min_cost = new_all_cost;
            self.min_id = id;
        } else if new_all_cost < self.second_min_cost {
            self.second_min_cost = new_all_cost;
        }
    }
}

impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        if costs.len() == 0 {
            return 0;
        }
        let mut record = CostRecord::new(0);
        for i in 0..costs.len() {
            let mut new_record = CostRecord::new(i32::max_value());
            for j in 0..costs[i].len() {
                new_record.add(&record, j, costs[i][j]);
            }
            record = new_record;
        }
        record.min_cost
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_cost_ii() {
        let costs = vec![vec![1,5,3],vec![2,9,4]];
        assert_eq!(Solution::min_cost_ii(costs), 5);
    }
}