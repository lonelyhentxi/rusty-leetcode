/*
 * @lc app=leetcode.cn id=408 lang=rust
 *
 * [408] 有效单词缩写
 */
// @lc code=start

#[derive(Debug)]
enum AbbrType {
    Char(char),
    Num(i32)
}

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let mut achars: Vec<AbbrType> = vec![];
        let mut current_num: Option<Vec<char>> = None;
        for a in abbr.chars() {
            if a.is_digit(10) {
                current_num = match &mut current_num {
                    Some(ref mut v) => { v.push(a); current_num },
                    None => Some(vec![a])
                }
            } else {
                if let Some(v) = current_num {
                    let (num, valid_num) = Solution::validate_and_transform_num(v);
                    if !valid_num {
                        return false;
                    }
                    achars.push(AbbrType::Num(num));
                    current_num = None;
                }
                achars.push(AbbrType::Char(a));
            }
        }
        if let Some(v) = current_num {
            let (num, valid_num) = Solution::validate_and_transform_num(v);
            if !valid_num {
                return false;
            }
            achars.push(AbbrType::Num(num));
        }
        let mut i = 0;
        let wchars = word.chars().collect::<Vec<char>>();
        let wlen = wchars.len() as i32;
        println!("{:?}", achars);
        for a in achars {
            i += match a {
                AbbrType::Char(ch) => {
                    if i >= wlen || ch != wchars[i as usize] {
                        return false;
                    }
                    1
                },
                AbbrType::Num(num) => {
                    for j in 0..num {
                        if i + j >= wlen {
                            return false;
                        }
                    }
                    num
                }
            }
        }
        i == wlen
    }

    fn validate_and_transform_num(num_chars: Vec<char>) -> (i32, bool) {
        let str: String = num_chars.iter().collect();
        let num = str.parse::<i32>().unwrap();
        (num, num!=0 && num.to_string() == str)
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_word_abbreviation_1() {
        assert!(Solution::valid_word_abbreviation(String::from("internationalization"), String::from("i12iz4n")));
    }

    #[test]
    fn test_valid_word_abbreviation_2() {
        assert!(!Solution::valid_word_abbreviation(String::from("apple"), String::from("a2e")));
    }

    #[test]
    fn test_valid_word_abbreviation_3() {
        assert!(!Solution::valid_word_abbreviation(String::from("hi"), String::from("1")));
    }
}