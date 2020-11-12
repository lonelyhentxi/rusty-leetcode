/*
 * @lc app=leetcode.cn id=642 lang=rust
 *
 * [642] 设计搜索自动补全系统
 */
// @lc code=start
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

#[derive(Clone, Debug)]
struct TrieNode {
    children: HashMap<char, Trie>,
    terms: usize
}

#[derive(Clone, Debug)]
struct Trie(Rc<RefCell<TrieNode>>);

impl Deref for Trie {
    type Target = Rc<RefCell<TrieNode>>;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}

impl DerefMut for Trie {
    fn deref_mut(self: &'_ mut Self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

impl Trie {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(TrieNode {
            children: HashMap::new(),
            terms: 0
        })))
    }

    pub fn add(&mut self, word: &[char], times: usize) {
        let mut node = self.borrow_mut();
        if word.is_empty() {
            node.terms += times;
        } else {
            let ch = word[0];
            if let Some(child_node) = node.children.get_mut(&ch) {
                child_node.add(&word[1..], times);
            } else {
                let mut new_node = Trie::new();
                new_node.add(&word[1..], times);
                node.children.insert(ch, new_node);
            }
        }
    }

    pub fn match_next(&self, ch: char) -> Option<Trie> {
        let node = self.borrow();
        if let Some(child_node) = node.children.get(&ch) {
            Some(child_node.clone())
        } else {
            None
        }
    }
}

impl IntoIterator for Trie {
    type Item = (Trie, Vec<char>);
    type IntoIter = TrieIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        TrieIntoIterator { queue: vec![(self, vec![])] }
    }
}

struct TrieIntoIterator {
    queue: Vec<(Trie, Vec<char>)>,
}

impl Iterator for TrieIntoIterator {
    type Item = (Trie, Vec<char>);

    fn next(&mut self) -> Option<(Trie, Vec<char>)> {
        while let Some((trie, s)) = self.queue.pop() {
            let is_term = {
                let node = trie.borrow_mut();
                for (k, c) in &node.children {
                    let mut ns = s.clone();
                    ns.push(*k);
                    self.queue.push((c.clone(), ns));
                }
                node.terms > 0
            };
            if is_term {
                return Some((trie, s));
            }
        }
        None
    }
}

struct AutocompleteSystem {
    root: Trie,
    sentence: Vec<char>,
    curr: Option<Trie>,
    to_show: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AutocompleteSystem {
    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        let sentence = vec![];
        let root = Trie::new();
        let mut acs = Self {
            root: root.clone(),
            sentence,
            curr: Some(root),
            to_show: 0,
        };
        let len = sentences.len();
        for i in 0..len {
            let chars = sentences[i].chars().collect::<Vec<_>>();
            acs.root.add(&chars, times[i] as usize);
        }
        acs
    }

    fn input(&mut self, c: char) -> Vec<String> {
        if c == '#' {
            self.root.add(&self.sentence, 1);
            self.sentence.clear();
            self.curr = Some(self.root.clone());
            vec![]
        } else {
            let mut res = vec![];
            self.sentence.push(c);
            let new_curr = if let Some(curr_trie) = self.curr.clone() {
                curr_trie.match_next(c)
            } else {
                None
            };
            if let Some(curr_trie) = new_curr.clone() {
                let mut heap = BinaryHeap::<(Reverse<usize>, Vec<char>)>::new();
                for (t, suffix) in curr_trie {
                    let n = t.borrow();
                    let mut prefix = self.sentence.clone();
                    prefix.extend_from_slice(&suffix[..]);
                    let new = (Reverse(n.terms), prefix);
                    if heap.len() < 3 {
                        heap.push(new);
                    } else {
                        let old = heap.pop().unwrap();
                        heap.push(if new < old { new } else { old });
                    }
                }
                res = heap.into_sorted_vec()
                    .into_iter()
                    .map(|(_, v)| v.into_iter().collect::<String>())
                    .collect::<Vec<_>>();
            }
            self.curr = new_curr;
            res
        }
    }
}

// @lc code=end


#[cfg(test)]
mod test {
    use super::*;
    use crate::lc_vec_s;

    #[test]
    fn test_autocomplete_system_1() {
        let mut acs = AutocompleteSystem::new(lc_vec_s!["i love you","island","iroman","i love leetcode"], vec![5, 3, 2, 2]);
        assert_eq!(acs.input('i'), lc_vec_s!["i love you", "island","i love leetcode"]);
        assert_eq!(acs.input(' '), lc_vec_s!["i love you","i love leetcode"]);
        assert_eq!(acs.input('a'), Vec::<String>::new());
        assert_eq!(acs.input('#'), Vec::<String>::new());
        assert_eq!(acs.input('i'), lc_vec_s!["i love you", "island","i love leetcode"]);
        assert_eq!(acs.input(' '), lc_vec_s!["i love you","i love leetcode", "i a"]);
        assert_eq!(acs.input('a'), lc_vec_s!["i a"]);
        assert_eq!(acs.input('#'), Vec::<String>::new());
        assert_eq!(acs.input('i'), lc_vec_s!["i love you", "island","i a"]);
        assert_eq!(acs.input(' '), lc_vec_s!["i love you","i a", "i love leetcode"]);
        assert_eq!(acs.input('a'), lc_vec_s!["i a"]);
        assert_eq!(acs.input('#'), Vec::<String>::new());
    }

    #[test]
    fn test_autocomplete_system_2() {
        let mut acs = AutocompleteSystem::new(lc_vec_s!["abc","abbc","a"], vec![3, 3, 3]);
        assert_eq!(acs.input('b'), Vec::<String>::new());
        assert_eq!(acs.input('c'), Vec::<String>::new());
        assert_eq!(acs.input('#'), Vec::<String>::new());
        assert_eq!(acs.input('b'), lc_vec_s!["bc"]);
    }
}