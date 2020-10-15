
const assert = require('assert');

// Definition for a Node.
function Node(val) {
  this.val = val;
  this.left = null;
  this.right = null;
  this.parent = null;
};

/*
 * @lc app=leetcode.cn id=510 lang=javascript
 *
 * [510] 二叉搜索树中的中序后继 II
 */

// @lc code=start

function treeMin (node) {
  let min = null;
  if(node) {
    if(!node.left) {
      min = node; 
    } else {
      min = treeMin(node.left);
    }
  }
  return min;
}

/**
 * @param {Node} node
 * @return {Node}
 */
var inorderSuccessor = function(node) {
    if(!node) {
      return null;
    } else {
      const rightMin = treeMin(node.right);
      if (!rightMin) {
        let curr = node;
        while (curr.parent && !Object.is(curr.parent.left, curr)) {
          curr = curr.parent;
        }
        if (!curr.parent) {
          return rightMin;
        } else {
          return curr.parent;
        }
      } else {
        return rightMin;
      }
    }
};
// @lc code=end

const test1 = () => {
  let tree = {};
  tree.left = { parent: tree };
  tree.right = { parent: tree };
  assert(Object.is(inorderSuccessor(tree.left), tree));
}

const test2 = () => {
  let tree = {};
  tree.left = { parent: tree };
  tree.right = { parent: tree };
  tree.left.left = { parent: tree.left };
  tree.left.right = { parent: tree.left };
  tree.left.left.left = { parent: tree.left.left };
  assert(Object.is(inorderSuccessor(tree.right), null));
}

const test3 = () => {
  let tree = { val: 15 };
  tree.left = { parent: tree, val: 6 };
  tree.right = { parent: tree, val: 18 };
  tree.left.left = { parent: tree.left, value: 3 };
  tree.left.right = { parent: tree.left, value: 7 };
  tree.left.left.left = { parent: tree.left.left, value: 2 };
  tree.left.left.right = { parent: tree.left.left, value: 4 };
  tree.left.right.right = { parent: tree.left.right, value: 13 };
  tree.left.right.right.left = { parent: tree.left.right.right, value: 9 };
  tree.right.left = { parent: tree.right, value: 17 };
  tree.right.right = { parent: tree.right, value: 20 };
  assert(Object.is(inorderSuccessor(tree), tree.right.left));
  assert(Object.is(inorderSuccessor(tree.left.right), tree.left.right.right.left));
}

test1();
test2();
test3();