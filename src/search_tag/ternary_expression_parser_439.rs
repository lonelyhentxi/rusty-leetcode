/*
 * @lc app=leetcode.cn id=439 lang=rust
 *
 * [439] 三元表达式解析器
 */

// @lc code=start
impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        let chars = expression.chars().collect::<Vec<_>>();
        if chars.len() <= 0 {
            return String::from("");
        }
        let mut stack = vec![];
        let mut i = (chars.len() -  1) as i32;
        while i >= 0 {
            let add = match chars[i as usize] {
                '?' => {
                    let condition = chars[(i - 1) as usize] == 'T';
                    i -= 1;
                    let left = stack.pop().unwrap();
                    stack.pop();
                    let right = stack.pop().unwrap();
                    if condition {
                        left
                    } else {
                        right
                    }
                },
                ch => ch
            };
            stack.push(add);
            i -= 1;
        }
        String::from(stack[0])
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_ternary_1() {
        assert_eq!(Solution::parse_ternary(String::from("T?2:3")), "2");
    }

    #[test]
    fn test_parse_ternary_2() {
        assert_eq!(Solution::parse_ternary(String::from("F?1:T?4:5")), "4");
    }

    #[test]
    fn test_parse_ternary_3() {
        assert_eq!(Solution::parse_ternary(String::from("T?T?F:5:3")), "F");
    }
}