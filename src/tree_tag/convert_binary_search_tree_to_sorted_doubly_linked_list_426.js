// Definition for a Node.
function Node(val, left, right) {
    this.val = val;
    this.left = left;
    this.right = right;
};

/*
 * @lc app=leetcode.cn id=426 lang=javascript
 *
 * [426] 将二叉搜索树转化为排序的双向链表
 */


// @lc code=start

/**
 * @param { Node! } root
 */
function treeToDoubleRecursive(root) {
    let leftRet;
    let rightRet;
    if (!root.left) {
        leftRet = root;
    } else {
        const leftRes = treeToDoubleRecursive(root.left);
        leftRes.right.right = root;
        root.left = leftRes.right;
        leftRet = leftRes.left;
    }
    if (!root.right) {
        rightRet = root;
    } else {
        const rightRes = treeToDoubleRecursive(root.right);
        rightRes.left.left = root;
        root.right = rightRes.left;
        rightRet = rightRes.right;
    }
    return { left: leftRet, right: rightRet };
}

/**
 * @param { Node } root 
 */
var treeToDoublyList = function(root) {
    if (!root) {
        return null;
    }
    const res = treeToDoubleRecursive(root);
    res.left.left = res.right;
    res.right.right = res.left;
    return res.left;
};
// @lc code=end