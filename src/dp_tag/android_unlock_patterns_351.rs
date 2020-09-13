/*
 * @lc app=leetcode.cn id=351 lang=rust
 *
 * [351] 安卓系统手势解锁
 */

// @lc code=start
use std::collections::{HashSet, HashMap};

static mut INDIRECT: Vec<HashMap<i32, i32>> = vec![];

fn try_init_indirect() -> & 'static Vec<HashMap<i32, i32>> {
    unsafe {
        if INDIRECT.is_empty() {
            INDIRECT = vec![
                vec![],
                vec![(2, 3), (4, 7), (5, 9)],
                vec![(5, 8)],
                vec![(2, 1), (6, 9), (5, 7)],
                vec![(5, 6)],
                vec![],
                vec![(5, 4)],
                vec![(4, 1), (8, 9), (5, 3)],
                vec![(5, 2)],
                vec![(6, 3), (8, 7), (5, 1)]
            ]
            .into_iter()
            .map(|v| v.into_iter().map(|(k, v)| (v, k))
            .collect::<HashMap<i32, i32>>()).collect();
        }
        &INDIRECT
    }
}

#[derive(Debug)]
struct Route {
    used: HashSet<i32>,
    curr: i32,
    unused: HashSet<i32>
}

impl Route {
    pub fn new() -> Self {
        Route {
            used: HashSet::new(),
            curr: 0,
            unused: (1..=9).collect::<HashSet<_>>()
        }
    }

    pub fn accessible(&self) -> Vec<Route> {
        let indirect = &try_init_indirect()[self.curr as usize];
        self.unused.iter().map(|i| if indirect.contains_key(&i) {
            let passthrough = indirect[&i];
            if self.used.contains(&passthrough) {
                Some(*i)
            } else {
                None
            }
        } else {
            Some(*i)
        })
        .filter(|s| s.is_some())
        .map(|target| self.next(target.unwrap()))
        .collect()
    }

    pub fn next(&self, target: i32) -> Self {
        let mut next_used = self.used.clone();
        next_used.insert(target);
        let mut next_unused = self.unused.clone();
        next_unused.remove(&target);
        return Route {
            used: next_used,
            curr: target,
            unused: next_unused
        }
    }
}

impl Solution {
    pub fn number_of_patterns(m: i32, n: i32) -> i32 {
        let mut rs = vec![Route::new()];
        let mut count = 0;
        while let Some(r) = rs.pop() {
            if r.used.len() as i32 >= m && r.used.len() as i32 <= n {
                count += 1;
            }
            if (r.used.len() as i32) < n {
                rs.extend(r.accessible());
            } 
        }
        count

    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_number_of_patterns_1() {
        assert_eq!(Solution::number_of_patterns(1, 1), 9);
    }

    #[test]
    fn test_number_of_patterns_2() {
        assert_eq!(Solution::number_of_patterns(1, 2), 65);
    }
}