/*
 * @lc app=leetcode.cn id=635 lang=rust
 *
 * [635] 设计日志存储系统
 */

// @lc code=start
use std::collections::BTreeMap;
use std::ops::Bound::Included;

const STARTS: [&'static str; 6] = ["0000", "00", "00", "00", "00", "00"];
const ENDS: [&'static str; 6] = ["9999", "12", "31", "23", "59", "59"];

struct LogSystem {
    logs: BTreeMap<u64, i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LogSystem {

    pub fn new() -> Self {
        Self {
            logs: BTreeMap::new()
        }
    }
    
    pub fn put(&mut self, id: i32, timestamp: String) {
        self.logs.insert(LogSystem::to_timestamp(&timestamp), id);
    }
    
    pub fn retrieve(&self, start: String, end: String, granularity: String) -> Vec<i32> {
        let level = LogSystem::granularity_to_level(&granularity);
        let from = LogSystem::to_start_timestamp(&start, level);
        let to = LogSystem::to_end_timestamp(&end, level);
        self.logs.range((Included(&from), Included(&to))).map(|(_, v)| *v).collect()
    }

    fn timestamp_to_chunks<'a>(ts: &'a str) -> Vec<&'a str> {
        ts.split(":").collect::<Vec<_>>()
    }

    fn granularity_to_level(granularity: &str) -> usize {
        match granularity {
            "Year" => 1,
            "Month" => 2,
            "Day" => 3,
            "Hour" => 4,
            "Minute" => 5,
            "Second" => 6,
            _ => unreachable!()
        }
    }

    fn to_timestamp(ts: &str) -> u64 {
        ts.replace(":","").parse::<u64>().unwrap()
    }

    fn to_start_timestamp(ts: &str, level: usize) -> u64 {
        let chunks = LogSystem::timestamp_to_chunks(ts);
        let mut res = String::new();
        for i in 0..level {
            res += chunks[i];
        }
        for i in level..chunks.len() {
            res += STARTS[i];
        }
        res.parse::<u64>().unwrap()
    }

    fn to_end_timestamp(ts: &str, level: usize) -> u64 {
        let chunks = LogSystem::timestamp_to_chunks(ts);
        let mut res = String::new();
        for i in 0..level {
            res += chunks[i];
        }
        for i in level..chunks.len() {
            res += ENDS[i];
        }
        res.parse::<u64>().unwrap()
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_log_system_1() {
        let mut ls = LogSystem::new();
        ls.put(1, String::from("2017:01:01:23:59:59"));
        ls.put(2, String::from("2017:01:01:22:59:59"));
        ls.put(3, String::from("2016:01:01:00:00:00"));
        assert_eq!(ls.retrieve(String::from("2016:01:01:01:01:01"), String::from("2017:01:01:23:00:00"), String::from("Year")), vec![3, 2, 1]);

        assert_eq!(ls.retrieve(String::from("2016:01:01:01:01:01"), String::from("2017:01:01:23:00:00"), String::from("Hour")), vec![2, 1])
    }
}