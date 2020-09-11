/*
 * @lc app=leetcode.cn id=346 lang=rust
 *
 * [346] 数据流中的移动平均值
 */

// @lc code=start
use std::collections::VecDeque;

struct MovingAverage {
    window: VecDeque<i32>,
    sum: i32,
    size: usize
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        MovingAverage {
            window: VecDeque::new(),
            sum: 0,
            size: size as usize
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.window.len() == self.size {
            if let Some(first) = self.window.pop_front() {
                self.sum -= first;
            } else {
                return 0 as f64;
            }
        }
        self.window.push_back(val);
        self.sum += val;
        (self.sum as f64) / (self.window.len() as f64)
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::assert_feq;

    #[test]
    fn test_moveing_average_1() {
        let mut m = MovingAverage::new(3);
        assert_feq(m.next(1), 1f64);
        assert_feq(m.next(10), ((1 + 10) as f64) / 2f64);
        assert_feq(m.next(3), ((1 + 10 + 3) as f64) / 3f64);
        assert_feq(m.next(5), ((10 + 3 + 5) as f64) / 3f64);
    }

    #[test]
    fn test_moveing_average_2() {
        let mut m = MovingAverage::new(0);
        assert_feq(m.next(1), 0f64);
        assert_feq(m.next(10), 0f64);
        assert_feq(m.next(3), 0f64);
        assert_feq(m.next(5), 0f64);
    }
}
