/*
 * @lc app=leetcode.cn id=716 lang=rust
 *
 * [716] 最大栈
 */

// @lc code=start
use std::collections::BTreeMap;

#[derive(Debug)]
struct MaxStack {
    stack: BTreeMap<usize, i32>,
    max: BTreeMap<(i32, usize), usize>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: BTreeMap::new(),
            max: BTreeMap::new()
        }
    }
    
    fn push(&mut self, x: i32) {
        let last = self.stack.iter().next_back();
        let new_index = if let Some((&i, _)) = last { i + 1 } else { 0 };
        self.stack.insert(new_index, x);
        self.max.insert((x, new_index), new_index);
    }
    
    fn pop(&mut self) -> i32 {
        let (&i, &v) = self.stack.iter().next_back().unwrap();
        self.stack.remove(&i);
        self.max.remove(&(v, i));
        v
    }
    
    fn top(&self) -> i32 {
        let (_, &v) = self.stack.iter().next_back().unwrap();
        v
    }
    
    fn peek_max(&self) -> i32 {
        let (&(v, _), _) = self.max.iter().next_back().unwrap();
        v
    }
    
    fn pop_max(&mut self) -> i32 {
        let (&(v, _), &i) = self.max.iter().next_back().unwrap();
        self.stack.remove(&i);
        self.max.remove(&(v, i));
        v
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_stack_1() {
        let mut stk = MaxStack::new();
        stk.push(5);
        stk.push(1);
        stk.push(5);
        assert_eq!(stk.top(), 5);
        assert_eq!(stk.pop_max(), 5);
        assert_eq!(stk.top(), 1);
        assert_eq!(stk.peek_max(), 5);
        assert_eq!(stk.pop(), 1);
        assert_eq!(stk.top(), 5);
    }

    #[test]
    fn test_max_stack_2() {
        let mut stk = MaxStack::new();
        stk.push(5);
        stk.push(1);
        assert_eq!(stk.pop_max(), 5);
        assert_eq!(stk.top(), 1);
    }
}