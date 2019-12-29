const { ListNode } = require('../utils/index');
const assert = require('assert')

/*
 * @lc app=leetcode.cn id=237 lang=javascript
 *
 * [237] 删除链表中的节点
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} node
 * @return {void} Do not return anything, modify node in-place instead.
 */
var deleteNode = function(node) {
    // assert(node.next!=null);
    if(node!=null) {
        node.val = node.next.val;
        node.next = node.next.next;
    }
};
// @lc code=end

function testDeleteNode1() {
    const list = ListNode.fromArray([4,5,1,9]);
    deleteNode(list.next);
    const expected = ListNode.fromArray([4,1,9]);
    assert(list.containerEquals(expected));
}

function testDeleteNode2() {
    const list = ListNode.fromArray([4,5,1,9]);
    deleteNode(list.next.next);
    const expected = ListNode.fromArray([4,5,9]);
    assert(list.containerEquals(expected));
}

if (!module.parent) {
    testDeleteNode1()
    testDeleteNode2()
}