/*
 * @lc app=leetcode.cn id=288 lang=rust
 *
 * [288] 单词的唯一缩写
 */

// @lc code=start
use std::collections::HashMap;

struct ValidWordAbbr {
    values:HashMap<String, (usize, String)>
}


impl ValidWordAbbr {

    fn new(dictionary: Vec<String>) -> Self {
        let mut dict: HashMap<String, (usize, String)> = HashMap::new();
        for s in dictionary {
            let abbr = ValidWordAbbr::to_abbr(&s);
            dict.entry(abbr)
                .and_modify(|c| {
                    if s!=c.1 {
                        c.0 += 1;
                    }
                })
                .or_insert_with(|| (1, s));
        }
        Self {
            values: dict
        }
    }
    
    fn is_unique(&self, word: String) -> bool {
        self.values.get(&ValidWordAbbr::to_abbr(&word))
            .map_or(true, |c| c.0==1 && c.1==word)
    }

    fn to_abbr(word: &str) -> String {
        if word.len() <= 2 {
            return word.to_string();
        }
        let chars = word.chars().collect::<Vec<char>>();
        let chars_last_i = chars.len() - 1;
        let mut res = String::new();
        res.push(chars[0]);
        res.extend(chars[1..chars_last_i].len().to_string().chars());
        res.push(chars[chars_last_i]);
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::map_to_string;

    #[test]
    fn test_valid_word_abbr() {
        let input = map_to_string(&[ "deer", "door", "cake", "card" ]);
        let validator = ValidWordAbbr::new(input);
        assert!(!validator.is_unique(String::from("dear")));
        assert!(validator.is_unique(String::from("cart")));
        assert!(!validator.is_unique(String::from("cane")));
        assert!(validator.is_unique(String::from("make")));
    }
}