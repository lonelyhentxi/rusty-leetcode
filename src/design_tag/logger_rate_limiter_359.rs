/*
 * @lc app=leetcode.cn id=359 lang=rust
 *
 * [359] 日志速率限制器
 */

// @lc code=start
use std::collections::{ HashSet, VecDeque };

struct Logger {
    unique: HashSet<String>,
    log: VecDeque<(i32, String)>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Logger {
            unique: HashSet::new(),
            log: VecDeque::new()
        }
    }
    
    /** Returns true if the message should be printed in the given timestamp, otherwise returns false.
        If this method returns false, the message will not be printed.
        The timestamp is in seconds granularity. */
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        while let Some((t, s)) = self.log.pop_front() {
            if t + 10 > timestamp {
                self.log.push_front((t, s));
                break;
            } else {
                self.unique.remove(&s);
            }
        }
        if self.unique.contains(&message) {
            false
        } else {
            self.unique.insert(message.clone());
            self.log.push_back((timestamp, message));
            true
        }
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logger_rate_limiter() {
        let actions: Vec<(i32, &str)> = 
            vec![(1, "foo"), (2, "bar"), (3, "foo"), (8, "bar"), (10, "foo"), (11, "foo")];
        let results: Vec<bool> = vec![true, true, false, false, false, true];
        let mut logger = Logger::new();

        for i in 0..actions.len() {
            assert_eq!(logger.should_print_message(actions[i].0, String::from(actions[i].1)), results[i])
        }
    }
}