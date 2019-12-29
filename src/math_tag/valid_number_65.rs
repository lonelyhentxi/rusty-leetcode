/*
 * @lc app=leetcode.cn id=65 lang=rust
 *
 * [65] 有效数字
 */
// @lc code=start

/**
 * -- states --
 * S0: ^ // start states
 * S1: [+-] // real sign
 * S2: \. // real point without digits before it
 * S3: \d // real digits before real point
 * S4: \. // real point with digits before it
 * S5: \d // real digits after real point
 * S6: [eE] // power init
 * S7: [+-] // power sign
 * S8: \d // power sign
 * S9: $ // end
 * S10: ! // err
 * -- transition --
 * S0 -> S1 | S2 | S3
 * S1 -> S2 | S3
 * S2 -> S5
 * S3 -> S3 | S4 | S6 | S9
 * S4 -> S5 | S6 | S9
 * S5 -> S5 | S6 | S9
 * S6 -> S7 | S8
 * S7 -> S8
 * S8 -> S8 | S9
 */

const ZERO_USZIE: usize = '0' as usize;
const NINE_USZIE: usize = '9' as usize;
const TRANSITIONS: [[usize;4];9] = [
    [1,2,3,10],
    [10,2,3,10],
    [10,10,5,10],
    [10,4,3,6],
    [10,10,5,6],
    [10,10,5,6],
    [7,10,8,10],
    [10,10,8,10],
    [10,10,8,10]
];
const STOPS: [usize;4] = [3,4,5,8];

impl Solution {
    #[inline]
    fn char_index(s: char) -> usize {
        match s {
            c if c=='+' || c=='-' => 0,
            c if c=='.' => 1 ,
            c if (c as usize)>=ZERO_USZIE && (c as usize)<=NINE_USZIE => 2,
            c if c=='e' || c=='E' => 3,
            _ => 4
        }
    }

    pub fn is_number(s: String) -> bool {
        let mut current_state = 0usize;
        for c in s.trim().chars() {
            if current_state==10 {
                return false;
            }
            let c_index = Solution::char_index(c);
            if c_index == 4 {
                return false;
            } else {
                current_state = TRANSITIONS[current_state][c_index];
            }
        }
        for stop in &STOPS {
            if *stop==current_state {
                return true;
            }
        }
        false
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::map_to_string;

    #[test]
    fn test_is_number() {
        let valid_examples = map_to_string(&[
            "0"," 0.1 ","2e10"," -90e3   "," 6e-1","53.5e93",
            " +1.25e-64"
            ]);
        let invalid_examples = map_to_string(&[
            "abc","1 a"," 1e","e3"," 99e2.5 "," --6 ","-+3","95a54e53"
        ]);
        assert_eq!(
            valid_examples.iter().map(|v| Solution::is_number(v.to_string())).collect::<Vec<_>>(),
            valid_examples.iter().map(|_| true).collect::<Vec<_>>(),
        );
        assert_eq!(
            invalid_examples.iter().map(|v| Solution::is_number(v.to_string())).collect::<Vec<_>>(),
            invalid_examples.iter().map(|_| false).collect::<Vec<_>>(),
        );
    }
}

