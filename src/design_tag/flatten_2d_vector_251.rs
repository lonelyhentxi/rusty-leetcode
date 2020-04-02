/*
 * @lc app=leetcode.cn id=251 lang=rust
 *
 * [251] 展开二维向量
 */

// @lc code=start
struct Vector2D {
    values: Vec<Vec<i32>>,
    i: usize,
    j: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Vector2D {
    fn new(v: Vec<Vec<i32>>) -> Self {
        Vector2D {
            values: v,
            i: 0,
            j: 0,
        }
    }

    fn next(&mut self) -> i32 {
        let has_next = self.has_next();
        if has_next {
            let res = self.values[self.i][self.j];
            self.j += 1;
            res
        } else {
            panic!("has not next")
        }
    }

    fn has_next(&mut self) -> bool {
        loop {
            if self.i>= self.values.len() {
                return false;
            } else if self.j < self.values[self.i].len() {
                return true;
            } else {
                self.i+=1;
                self.j = 0;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_2d() {
        let vv = vec![vec![1,2],vec![3],vec![4]];
        let mut v2d = Vector2D::new(vv);
        assert_eq!(v2d.next(), 1);
        assert_eq!(v2d.next(), 2);
        assert_eq!(v2d.next(), 3);
        assert!(v2d.has_next());
        assert!(v2d.has_next());
        assert_eq!(v2d.next(), 4);
        assert!(!v2d.has_next());
    }
}
