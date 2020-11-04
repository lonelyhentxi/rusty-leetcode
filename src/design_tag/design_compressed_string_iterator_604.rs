/*
 * @lc app=leetcode.cn id=604 lang=rust
 *
 * [604] 迭代压缩字符串
 */

// @lc code=start

#[derive(Clone, Debug)]
enum StringIteratorState {
    Unknown,
    Current { content: char, repeat: usize },
    Empty,
}

#[derive(Clone, Debug)]
struct StringIterator {
    source: Vec<char>,
    cursor: usize,
    state: StringIteratorState,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StringIterator {
    pub fn new(compressed_string: String) -> Self { 
        let mut source = compressed_string.chars().collect::<Vec<_>>();
        source.push('#');
        Self {
            source,
            cursor: 0,
            state: StringIteratorState::Unknown,
        }
    }

    pub fn consume_next(&mut self) {
        if let StringIteratorState::Unknown = self.state {
            let mut i = self.cursor;
            let ch = self.source[i];
            self.state = if ch == '#' {
                StringIteratorState::Empty
            } else {
                let content = ch;
                let mut num_str = String::new();
                while i < self.source.len() {
                    i += 1;
                    let c = self.source[i];
                    if c.is_digit(10) {
                        num_str.push(c);
                    } else {
                        break;
                    }
                }
                self.cursor = i;
                StringIteratorState::Current {
                    content,
                    repeat: num_str.parse::<usize>().unwrap(),
                }
            };
        }
    }

    pub fn next(&mut self) -> char {
        self.consume_next();
        let res = match self.state {
            StringIteratorState::Empty => ' ',
            StringIteratorState::Current { content, repeat: _ } => content,
            _ => unreachable!(),
        };
        if let StringIteratorState::Current { content, repeat } = self.state {
            self.state = if repeat == 1 {
                StringIteratorState::Unknown
            } else {
                StringIteratorState::Current {
                    content,
                    repeat: repeat - 1,
                }
            };
        };
        res
    }

    pub fn has_next(&mut self) -> bool {
        self.consume_next();
        match self.state {
            StringIteratorState::Empty => false,
            StringIteratorState::Current {
                content: _,
                repeat: _,
            } => true,
            _ => unreachable!(),
        }
    }
}

// @lc code=end


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_iterator_1() {
        let mut iter = StringIterator::new(String::from("L1e2t1C1o1d1e1"));
        let res = vec!['L','e','e', 't','C','o','d'];
        for r in res {
            assert_eq!(iter.next(), r);
        }
        assert_eq!(iter.has_next(), true);
        assert_eq!(iter.next(), 'e');
        assert_eq!(iter.has_next(), false);
        assert_eq!(iter.next(), ' ');
    }
}