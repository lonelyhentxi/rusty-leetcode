const assert = require('assert');
const {TreeNode} = require('./lowest_common_ancestor_of_binary_tree_236');
/*
 * @lc app=leetcode.cn id=285 lang=javascript
 *
 * [285] 二叉搜索树中的顺序后继
 */


// @lc code=start
const mostLeft = function(node) {
    return node.left?mostLeft(node.left):node;
}

const toPPath = function(root, p) {
    const stack  = [];
    const  pv = p.val;
    let curr = root;
    while(pv!=curr.val) {
        stack.push(curr);
        if(pv>curr.val) {
            curr = curr.right;
        } else {
            curr = curr.left;
        }
    }
    return stack;
}

const inorderSuccessor = function(root, p) {
    if(p===null) {
        return null;
    }
    if(p.right!==null) {
        return mostLeft(p.right);
    }
    const path = toPPath(root, p);
    let curr = p;
    while(path.length>0) {
        const parent = path.pop();
        if(Object.is(curr, parent.left)) {
            return parent;
        } 
        curr = parent;
    }
    return null;
};
// @lc code=end

const test1 = function() {
    let node = new TreeNode(2);
    node.left = new TreeNode(1);
    node.right = new TreeNode(3);
    assert(Object.is(node, inorderSuccessor(node, node.left)));
}

test1();