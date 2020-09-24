use crate::utils::linked_list::ListNode;

/*
 * @lc app=leetcode.cn id=369 lang=rust
 *
 * [369] 给单链表加一
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::borrow::BorrowMut;

impl Solution {
    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let inc = match head {
            Some(ref mut head_node) => Solution::plus_one_recursive(head_node),
            None => false
        };
        if inc {
            Some(Box::new({ let mut new_head = ListNode::new(1); new_head.next = head; new_head }))
        } else {
            head
        }
    }

    pub fn plus_one_recursive(mut head: &mut Box<ListNode>) -> bool {
        let next = head.next.borrow_mut();
        let inc = match next {
            Some(ref mut next_ref) => Solution::plus_one_recursive(next_ref),
            None => true
        };
        if inc {
            head.val += 1;
        }
        if head.val == 10 {
            head.val = 0;
            true
        } else {
            false
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::linked_list::ListNode;

    #[test]
    fn test_plus_one() {
        let mut input = ListNode::from_doubled_iter(vec![1, 2, 3].into_iter());
        let output = ListNode::from_doubled_iter(vec![1, 2, 4].into_iter());
        input = Solution::plus_one(input);
        assert_eq!(input, output);
    }

    #[test]
    fn test_plus_one_1() {
        let mut input = ListNode::from_doubled_iter(vec![9].into_iter());
        let output = ListNode::from_doubled_iter(vec![1, 0].into_iter());
        input = Solution::plus_one(input);
        assert_eq!(input, output);
    }
}