/*
 * @lc app=leetcode.cn id=362 lang=rust
 *
 * [362] 敲击计数器
 */

// @lc code=start

#[derive(Debug)]
struct HitCounter {
  roll: Vec<i32>,
  count: i32,
  last_updated: i32,
  first_active: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl HitCounter {

    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self {
          roll: vec![0; 300],
          count: 0,
          last_updated: 0,
          first_active: 0,
        }
    }
    
    /** Record a hit.
        @param timestamp - The current timestamp (in seconds granularity). */
    pub fn hit(&mut self, timestamp: i32) {
      self.clear(timestamp);
      self.roll[((timestamp - 1) % 300) as usize] += 1;
      self.count += 1;
      self.last_updated = timestamp;
    }
    
    /** Return the number of hits in the past 5 minutes.
        @param timestamp - The current timestamp (in seconds granularity). */
    pub fn get_hits(&mut self, timestamp: i32) -> i32 {
      self.clear(timestamp);
      return self.count;
    }

    fn clear(&mut self, timestamp: i32) {
      let last_updated = self.last_updated;
      let first_active = self.first_active;
      if timestamp >= last_updated + 300 {
        self.roll = vec![0; 300];
        self.count = 0;
      } else if timestamp - 300 >= first_active + 1 {
        for t in (first_active + 1)..=(timestamp - 300) {
          let index = ((t - 1 + 300) % 300) as usize;
          self.count -= self.roll[index];
          self.roll[index] = 0;
        }
      }
      self.first_active = i32::max(0, timestamp - 300);
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hit_counter_1() {
      let mut counter = HitCounter::new();
      counter.hit(1);
      counter.hit(2);
      counter.hit(3);
      assert_eq!(counter.get_hits(4), 3);
      counter.hit(300);
      assert_eq!(counter.get_hits(300), 4);
      assert_eq!(counter.get_hits(301), 3);
    }

    #[test]
    fn test_hit_counter_2() {
      let mut counter = HitCounter::new();
      counter.hit(100);
      counter.hit(101);
      counter.hit(202);
      assert_eq!(counter.get_hits(310), 3);
      assert_eq!(counter.get_hits(400), 2);
      assert_eq!(counter.get_hits(401), 1);
    }
}