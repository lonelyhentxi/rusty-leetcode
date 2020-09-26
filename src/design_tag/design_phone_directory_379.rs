/*
 * @lc app=leetcode.cn id=379 lang=rust
 *
 * [379] 电话目录管理系统
 */

// @lc code=start
use std::collections::HashSet;

struct PhoneDirectory {
    size: usize,
    unused: HashSet<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl PhoneDirectory {

    /** Initialize your data structure here
        @param maxNumbers - The maximum numbers that can be stored in the phone directory. */
    fn new(max_numbers: i32) -> Self {
        Self {
            size: max_numbers as usize,
            unused: (0..max_numbers).collect()
        }
    }
    
    /** Provide a number which is not assigned to anyone.
        @return - Return an available number. Return -1 if none is available. */
    fn get(&mut self) -> i32 {
        let mut res = -1;
        for i in &self.unused {
            res = *i;
            break;
        }
        if res != -1 {
            self.unused.remove(&res);
        }
        return res;
    }
    
    /** Check if a number is available or not. */
    fn check(&self, number: i32) -> bool {
        self.unused.contains(&number)
    }
    
    /** Recycle or release a number. */
    fn release(&mut self, number: i32) {
        self.unused.insert(number);
    }
}

// @lc code=end