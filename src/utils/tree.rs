use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn build_tree(prefix: Vec<i32>, infix: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        use crate::tree_tag::construct_binary_tree_from_preorder_and_inorder_traversal_105::Solution;
        Solution::build_tree(prefix, infix)
    }

    pub fn build_lc_tree(tree_vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
      let len = tree_vec.len();
      if len == 0 {
        return None;
      }
      let root = tree_vec[0].map(|v| Rc::new(RefCell::new(TreeNode::new(v))));
      {
        let mut working = VecDeque::new();
        working.push_back(root.clone());
        let mut is_left = true;
        for i in 1..len {
          while let Some(curr) = working.pop_front() {
            if let Some(curr_ref) = curr.clone() {
              let mut curr_node = curr_ref.borrow_mut();
              let new_node = tree_vec[i].map(|v| Rc::new(RefCell::new(TreeNode::new(v))));
              if is_left {
                curr_node.left = new_node.clone();
                is_left = false;
                working.push_front(curr);
              } else {
                curr_node.right = new_node.clone();
                is_left = true;
              }
              working.push_back(new_node);
              break;
            }
          }
        }
      }
      root.clone()
    }
}

#[macro_export]
macro_rules! tree_node {
    ($elem: expr, $left: expr, $right: expr) => {
        Some(::std::rc::Rc::new(::std::cell::RefCell::new(crate::utils::tree::TreeNode {
            val: $elem,
            left: $left,
            right: $right,
        })))
    };
}

#[macro_export]
macro_rules! tree_leaf {
    ($elem: expr) => {
        tree_node!($elem, None, None)
    };
}

#[macro_export]
macro_rules! lc_tree_vec {
  () => {
    Vec::<Option<i32>>::new()
  };
  ( $x:expr ) => {
    {
      let mut temp_vec = Vec::<Option<i32>>::new();
      temp_vec.push( Some( $x ) );
      temp_vec
    }
  };
  ( null ,  $($rest: tt),* ) => {
    {
      let mut temp_vec = Vec::<Option<i32>>::new();
      temp_vec.push(None);
      temp_vec.extend( ( crate::lc_tree_vec![ $($rest),*  ] ).iter());
      temp_vec
    }
  };
  ( $x:expr ,  $($rest:tt),*  ) => {
    {
      let mut temp_vec = Vec::<Option<i32>>::new();
      temp_vec.push( Some( $x ) );
      temp_vec.extend( ( crate::lc_tree_vec![ $($rest),* ]  ).iter());
      temp_vec
    }
  };
}

#[macro_export]
macro_rules! lc_tree {
  ( $($rest: tt),* ) => {
    {
      crate::utils::tree::TreeNode::build_lc_tree( crate::lc_tree_vec![ $($rest),* ] )
    }
  };
}

#[cfg(test)]
mod test {
    use crate::{ lc_tree_vec, tree_node, tree_leaf, lc_tree };
    use super::*;

    #[test]
    fn test_lc_tree_vec_macro_1() {
        let i = lc_tree_vec![2, null, 2];
        let o = vec![Some(2), None, Some(2)];
        assert_eq!(i, o);
    }

    #[test]
    fn test_build_lc_tree_1() {
      let i = vec![Some(1), Some(2), Some(3), Some(4), None, Some(5)];
      let o = tree_node!(
        1,
        tree_node!(
          2,
          tree_leaf!(4),
          None
        ),
        tree_node!(3,
          tree_leaf!(5),
          None
        )
      );
      assert_eq!(TreeNode::build_lc_tree(i), o);
    }

    #[test]
    fn test_lc_tree_macro_1() {
      let i = lc_tree![1, 2, 3, 4, null, 5];
      let o = tree_node!(
        1,
        tree_node!(
          2,
          tree_leaf!(4),
          None
        ),
        tree_node!(3,
          tree_leaf!(5),
          None
        )
      );
      assert_eq!(i, o);
    }
}
