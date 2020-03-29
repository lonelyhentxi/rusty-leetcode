/*
 * @lc app=leetcode.cn id=170 lang=rust
 *
 * [170] 两数之和 III - 数据结构设计
 */

// @lc code=start

struct TwoSum {
    values: Vec<i32>,
    counts: Vec<usize>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {

    /** Initialize your data structure here. */
    fn new() -> Self {
        TwoSum {
            values: vec![],
            counts: vec![],
        }
    }
    
    #[allow(clippy::comparison_chain)]
    /** Add the number to an internal data structure.. */
    fn add(&mut self, number: i32) {
        let mut start = 0;
        let mut end = self.values.len();
        while start < end {
            let mid = (start + end)/2;
            let mid_value = self.values[mid];
            if number < mid_value {
                end = mid;
            } else if number > mid_value {
                start = mid + 1;
            } else {
                self.counts[mid]+=1;
                break;
            }
        }
        if start==end {
            self.values.insert(start, number);
            self.counts.insert(start, 1);
        }
    }
    
    #[allow(clippy::comparison_chain)]
    /** Find if there exists any pair of numbers which sum is equal to the value. */
    fn find(&self, value: i32) -> bool {
        let mut i = 0i32;
        let mut j = (self.values.len() as i32)-1;
        while i<j {
            let sum = self.values[i as usize]
                + self.values[j as usize];
            if sum==value {
                return true;
            } else if sum>value {
                j-=1;
            } else {
                i+=1;
            }
        }
        i==j 
        && self.counts[i as usize] >=2usize 
        && self.values[i as usize] * 2==value
    }
}

 // @lc code=end

 #[cfg(test)]
 mod tests {
     use super::*;
     #[test]
     fn test_two_sum_data_structure1() {
         let mut two_sum = TwoSum::new();
         two_sum.add(1);
         two_sum.add(3);
         two_sum.add(5);
         assert!(two_sum.find(4));
         assert!(!two_sum.find(7));
     }

     #[test]
     fn test_two_sum_data_structure2() {
         let mut two_sum = TwoSum::new();
         two_sum.add(3);
         two_sum.add(1);
         two_sum.add(2);
         assert!(two_sum.find(3));
         assert!(!two_sum.find(6));
     }

     #[test]
     fn test_two_sum_data_structure3() {
         let mut two_sum = TwoSum::new();
         two_sum.add(2);
         two_sum.add(3);
         two_sum.add(2);
         two_sum.add(5);
         assert!(two_sum.find(4));
     }
 }