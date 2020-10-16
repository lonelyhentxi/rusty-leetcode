/*
 * @lc app=leetcode.cn id=527 lang=rust
 *
 * [527] 单词缩写
 */
// @lc code=start
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::swap;
use std::rc::Rc;

const ACHARCODE: usize = 'a' as usize;

#[derive(Debug)]
struct Trie {
    children: HashMap<char, Rc<RefCell<Trie>>>,
    key: Vec<char>,
    is_leaf: bool
}

impl Trie {
    pub fn root() -> Self {
        Self {
            children: HashMap::<char, Rc<RefCell<Trie>>>::new(),
            key: vec![],
            is_leaf: false
        }
    }

    pub fn new(key: &[char], is_leaf: bool) -> Self {
        Self {
            children: HashMap::<char, Rc<RefCell<Trie>>>::new(),
            key: key.to_vec(),
            is_leaf: is_leaf
        }
    }

    pub fn add(&mut self, word: &[char]) {
        let mut i = 0;
        while i < word.len() && i < self.key.len() && word[i] == self.key[i] {
            i += 1;
        }
        if self.key.len() != i {
            let mut old_key = vec![];
            swap(&mut old_key, &mut self.key);
            self.key = old_key[0..i].to_vec();
            let mut derived = Trie::new(&old_key[i..], self.is_leaf);
            swap(&mut derived.children, &mut self.children);
            self.children.insert(old_key[i], Rc::new(RefCell::new(derived)));
            self.is_leaf = false;
        }
        if word.len() == i {
            self.is_leaf = true;
        } else {
            if let Some(sub_ref) = self.children.get_mut(&word[i]) {
                let mut sub_node = sub_ref.borrow_mut();
                sub_node.add(&word[i..]);
            } else {
                let new_leaf = Trie::new(&word[i..], true);
                self.children.insert(word[i],Rc::new(RefCell::new(new_leaf)));
            }
        }
    }

    fn to_abbr_rec(&self, word: &[char], unique: bool) -> usize {
        let mut i = 0;
        while i < word.len() && i < self.key.len() && word[i] == self.key[i] {
            i += 1;
        }
        if word.len() == i {
            self.key.len() + if unique { 1 } else { 0 } - 1
        } else {
            match self.children.get(&word[i]) {
                Some(sub_ref) => {
                    let sub_node = sub_ref.borrow();
                    sub_node.to_abbr_rec(&word[i..], self.children.len() == 1)
                }
                _ => {
                    unreachable!()
                },
            }
        }
    }

    pub fn to_abbr(&self, word: &[char]) -> usize {
        self.to_abbr_rec(word, true)
    }
}

impl Solution {
    pub fn words_abbreviation(dict: Vec<String>) -> Vec<String> {
        let dict = dict
            .into_iter()
            .map(|v| v.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let mut abbrs = HashMap::<(char, char, usize), Trie>::new();
        for d in &dict {
            let inner = &d[1..d.len() - 1];
            let ht = (d[0], d[d.len() - 1], inner.len());
            abbrs
                .entry(ht)
                .and_modify(|t| t.add(inner))
                .or_insert_with(|| {
                    let mut t = Trie::root();
                    t.add(inner);
                    t
                });
        }
        dict.into_iter()
            .map(|d| {
                let inner = &d[1..d.len() - 1];
                let ht = (d[0], d[d.len() - 1], inner.len());
                let mut abbr = String::new();
                abbr.push(d[0]);
                let abbr_len = abbrs[&ht].to_abbr(inner);
                for c in &d[1..(d.len() - 1 - abbr_len)] {
                    abbr.push(*c);
                }
                if abbr_len == 0 {
                    abbr += "";
                } else if abbr_len == 1 {
                    abbr.push(d[d.len() - 2]);
                } else {
                    abbr += &abbr_len.to_string();
                };
                abbr.push(d[d.len() - 1]);
                abbr
            })
            .collect::<Vec<_>>()
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::map_to_string;

    #[test]
    fn test_words_abbreviation_1() {
        let i = map_to_string(&[
            "like",
            "god",
            "internal",
            "me",
            "internet",
            "interval",
            "intension",
            "face",
            "intrusion",
        ]);
        let o = map_to_string(&[
            "l2e", "god", "internal", "me", "i6t", "interval", "inte4n", "f2e", "intr4n",
        ]);
        assert_eq!(Solution::words_abbreviation(i), o);
    }

    #[test]
    fn test_words_abbreviation_2() {
        let i = map_to_string(&[
            "abcdefg","abccefg","abcckkg"
        ]);
        let o = map_to_string(&[
            "abcd2g","abccefg","abcckkg"
        ]);
        assert_eq!(Solution::words_abbreviation(i), o);
    }

    #[test]
    fn test_words_abbreviation_3() {
        let i = map_to_string(&[
            "met","meet"
        ]);
        let o = map_to_string(&[
            "met","m2t"
        ]);
        assert_eq!(Solution::words_abbreviation(i), o);
    }
}