/*
 * @lc app=leetcode.cn id=631 lang=rust
 *
 * [631] 设计 Excel 求和公式
 */

// @lc code=start
use std::collections::HashMap;

const A_CHAR_CODE: usize = 'A' as u8 as usize;

#[derive(Clone)]
struct ExcelNode {
    value: i32,
    watches: Vec<Vec<(usize, usize)>>,
    observers: HashMap<(usize, usize), usize>,
}

impl ExcelNode {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            watches: vec![],
            observers: HashMap::new(),
        }
    }

    pub fn watches_to_vec(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for s in &self.watches {
            if s.len() == 1 {
                res.push(s[0]);
            } else {
                let (fr, fc) = s[0];
                let (tr, tc) = s[1];
                for r in fr..=tr {
                    for c in fc..=tc {
                        res.push((r, c));
                    }
                }
            }
        }
        res
    }

    pub fn observers_to_vec(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (&k, &v) in &self.observers {
            for _ in 0..v {
                res.push(k);
            }
        }
        res
    }
}

struct Excel {
    values: Vec<Vec<ExcelNode>>,
    rows: usize,
    cols: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Excel {
    pub fn new(h: i32, w: char) -> Self {
        let cols = h as usize;
        let rows = Excel::col_c2i(w as u8) + 1;
        Self {
            values: vec![vec![ExcelNode::new(0); cols]; rows],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, r: i32, c: char, v: i32) {
        let col = Excel::col_c2i(c as u8);
        let row = Excel::row_i2i(r);
        self.set_impl(row, col, v);
        self.clear_watches(row, col);
    }

    pub fn get(&self, r: i32, c: char) -> i32 {
        let col = Excel::col_c2i(c as u8);
        let row = Excel::row_i2i(r);
        self.retrieve(row, col).value
    }

    pub fn sum(&mut self, r: i32, c: char, strs: Vec<String>) -> i32 {
        let col = Excel::col_c2i(c as u8);
        let row = Excel::row_i2i(r);
        self.set_impl(row, col, 0);
        self.clear_watches(row, col);
        self.add_watches(row, col, strs);
        self.retrieve(row, col).value
    }

    fn set_impl(&mut self, row: usize, col: usize, v: i32) {
        let sub_v = v - self.retrieve(row, col).value;
        let mut stack = vec![(row, col)];
        while let Some((r, c)) = stack.pop() {
            let mut node = self.retrieve_mut(r, c);
            node.value += sub_v;
            stack.extend(node.observers_to_vec().into_iter());
        }
    }

    fn retrieve_mut(&mut self, r: usize, c: usize) -> &mut ExcelNode {
        &mut self.values[r][c]
    }

    fn retrieve(&self, r: usize, c: usize) -> &ExcelNode {
        &self.values[r][c]
    }

    fn clear_watches(&mut self, row: usize, col: usize) {
        let node = self.retrieve_mut(row, col);
        let watches = node.watches_to_vec();
        node.watches = vec![];

        let node_index = (row, col);
        for (r, c) in watches {
            let another = self.retrieve_mut(r, c);
            if another.observers[&node_index] == 1 {
                another.observers.remove(&node_index);
            } else {
                *another.observers.get_mut(&node_index).unwrap() -= 1;
            }
        }
    }

    fn add_watches(&mut self, row: usize, col: usize, strs: Vec<String>) {
        let node = self.retrieve_mut(row, col);
        for s in strs {
            let splits = s.split(":").collect::<Vec<_>>();
            if splits.len() == 1 {
                let sp = Excel::colrow_to_index(&splits[0]);
                node.watches.push(vec![sp]);
            } else {
                let fp = Excel::colrow_to_index(&splits[0]);
                let tp = Excel::colrow_to_index(&splits[1]);
                node.watches.push(vec![fp, tp]);
            }
        }
        let watches = node.watches_to_vec();
        let node_index = (row, col);
        let mut sum = 0;
        for (r, c) in watches {
            let another = self.retrieve_mut(r, c);
            another.observers.entry(node_index)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            sum += another.value;
        }
        self.set_impl(row, col, sum);
    }

    #[inline]
    fn col_c2i(c: u8) -> usize {
        c as usize - A_CHAR_CODE
    }

    #[inline]
    fn row_s2i(c: &str) -> usize {
        c.parse::<usize>().unwrap() - 1
    }

    #[inline]
    fn row_i2i(i: i32) -> usize {
        (i - 1) as usize
    }

    #[inline]
    fn colrow_to_index(colrow: &str) -> (usize, usize) {
        let col = Excel::col_c2i(colrow.as_bytes()[0]);
        let row = Excel::row_s2i(&colrow[1..]);
        (row, col)
    }

    fn to_matrix(&self) -> Vec<Vec<i32>> {
        self.values
            .iter()
            .map(|v| v.iter().map(|n| n.value).collect::<Vec<i32>>())
            .collect::<Vec<_>>()
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ lc_vec_s, lc_matrix };

    #[test]
    fn test_excel_1() {
        let mut excel = Excel::new(3, 'C');
        assert_eq!(excel.to_matrix(), lc_matrix![
            [0,0,0],
            [0,0,0],
            [0,0,0]
        ]);
        excel.set(1, 'A', 2);
        assert_eq!(excel.to_matrix(), lc_matrix![
            [2,0,0],
            [0,0,0],
            [0,0,0]
        ]);
        assert_eq!(excel.sum(3, 'C', lc_vec_s!["A1", "A1:B2"]), 4);
        assert_eq!(excel.to_matrix(), lc_matrix![
            [2,0,0],
            [0,0,0],
            [0,0,4]
        ]);
        excel.set(2, 'B', 2);
        assert_eq!(excel.to_matrix(), lc_matrix![
            [2,0,0],
            [0,2,0],
            [0,0,6]
        ]);
    }

    #[test]
    fn test_excel_2() {
        let mut excel = Excel::new(5, 'E');
        assert_eq!(excel.get(1, 'A'), 0);
        excel.set(1, 'A', 1);
        assert_eq!(excel.get(1, 'A'), 1);
        assert_eq!(excel.sum(2, 'B', lc_vec_s!["A1", "A1"]), 2);
        excel.set(1, 'A', 2);
        assert_eq!(excel.get(2, 'B'), 4);
    }
}
