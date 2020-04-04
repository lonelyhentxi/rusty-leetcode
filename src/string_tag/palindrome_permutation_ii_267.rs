/*
 * @lc app=leetcode.cn id=267 lang=rust
 *
 * [267] 回文排列 II
 */

// @lc code=start
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type CharCounter = HashMap<char, usize>;


pub fn count(counter: &mut CharCounter, ch: char) {
    counter.entry(ch)
        .and_modify(|c| *c+=1)
        .or_insert(1);
}

pub fn decount(counter: &mut CharCounter, ch: char, times: usize) {
    let count = *counter.get(&ch).unwrap();
    if times>=count {
        counter.remove(&ch);
    } else {
        counter.insert(ch, count-times);
    }
}

impl Solution {
    pub fn generate_palindromes(s: String) -> Vec<String> {
        let mut counter = CharCounter::new();
        for c in s.chars() {
            count(&mut counter,c);
        }
        let mut odd_center: Option<char> = None;
        if (s.len() & 1)==1 {
            for (k,v) in &counter {
                if (v&1)==1 {
                    match odd_center {
                        None => odd_center = Some(*k),
                        _ => return vec![],
                    }
                }
            }
        } else {
            for (_,v) in &counter {
                if (v&1)==1 {
                    return vec![];
                }
            }
        }
        if let Some(odd_char) = &odd_center {
            decount(&mut counter, *odd_char, 1);
        }
        let res_rc = Rc::new(RefCell::new(vec![]));
        Solution::generate_palindromes_rec(res_rc.clone(), counter, String::from(""));
        let src = res_rc.borrow();
        let mut res= vec![];
        if let Some(odd_char) = &odd_center {
            for r in src.iter() {
                let mut new_r = r.clone();
                new_r.push(*odd_char);
                new_r.extend(r.chars().rev());
                res.push(new_r);
            }
        } else {
            for r in src.iter() {
                let mut new_r = r.clone();
                new_r.extend(r.chars().rev());
                res.push(new_r);
            }
        }
        res
    }

    fn generate_palindromes_rec(
        res: Rc<RefCell<Vec<String>>>,
        counter: CharCounter,
        visited: String,
    ) {
        if counter.len()==0 {
            let mut res_mb = res.borrow_mut();
            res_mb.push(visited);
        } else {
            for k in counter.keys() {
                let mut new_visited = visited.clone();
                new_visited.push(*k);
                let mut new_counter = counter.clone();
                decount(&mut new_counter,*k, 2);
                Solution::generate_palindromes_rec(res.clone(), new_counter, new_visited);
            }
        }
    }
}
// @lc code=end

struct Solution;


#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::{map_to_string,assert_equivalent};

    #[test]
    fn test_generate_palindromes() {
        let target = map_to_string(&["abba", "baab"]);
        assert_equivalent(
            &Solution::generate_palindromes(String::from("aabb")),
            &target);
        assert_equivalent(
            &Solution::generate_palindromes(String::from("aba")),
            &vec![String::from("aba")]);
    }
}