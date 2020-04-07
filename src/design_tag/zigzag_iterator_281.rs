/*
 * @lc app=leetcode.cn id=281 lang=rust
 *
 * [281] 锯齿迭代器
 */

// @lc code=start
struct ZigzagIterator {
    vs: Vec<Vec<i32>>,
    i: usize,
    j: usize,
    ei: usize,
}

impl ZigzagIterator {
    /** initialize your data structure here. */
    
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let vs = vec![v1, v2];
        let mut end_line = 0;
        let mut line_max = usize::min_value();
        for (i, v) in vs.iter().enumerate() {
            if v.len() >= line_max {
                line_max = v.len();
                end_line = i;
            }
        }
        ZigzagIterator {
            vs, 
            i: 0,
            j: 0,
            ei: end_line
        }
    }
    
    fn next(&mut self) -> i32 {
        self.has_next();
        let res = self.vs[self.i][self.j];
        self.i += 1;
        res
    }
    
    fn has_next(&mut self) -> bool {
        let line_max = self.vs[self.ei].len();
        loop {
            if (self.i > self.ei && self.j + 1 == line_max) || self.j >= line_max {
                return false;
            } else if self.i<self.vs.len() {
                if self.j < self.vs[self.i].len()  {
                    return true;
                } else {
                    self.i += 1;
                }
            } 
            if self.i>=self.vs.len()  {
                self.i %= self.vs.len();
                self.j += 1;
            }
        }
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_iterator1() {
        let mut zz = ZigzagIterator::new(vec![1,2], vec![3,4,5,6]);
        let mut res = vec![];
        while zz.has_next() {
            res.push(zz.next());
        }
        assert_eq!(res, vec![1,3,2,4,5,6]);
    }

    #[test]
    fn test_zigzag_iterator2() {
        let mut zz = ZigzagIterator::new(vec![], vec![]);
        let mut res = vec![];
        while zz.has_next() {
            res.push(zz.next());
        }
        assert_eq!(res, vec![]);
    }
}