const assert = require('assert');

function Node(val, next, random) {
    this.val = val;
    this.next = next;
    this.random = random;
};

/*
 * @lc app=leetcode.cn id=138 lang=javascript
 *
 * [138] 复制带随机指针的链表
 */

// @lc code=start
/**
 * // Definition for a Node.
 * function Node(val, next, random) {
 *    this.val = val;
 *    this.next = next;
 *    this.random = random;
 * };
 */
/**
 * @param {Node} head
 * @return {Node}
 */
var copyRandomList = function(head) {
    let current = head;
    while(current!=null) {
        const oldNext = current.next;
        current.next = new Node(current.val,null,null);
        current.next.next = oldNext;
        current = oldNext;
    }
    current = head;
    while(current!=null) {
        current.next.random = current.random?current.random.next:null;
        current = current.next?current.next.next:null;
    }
    const newFakeHead = new Node(Infinity,null,null);
    current = head;
    let newCurrent = newFakeHead;
    while(current!=null) {
        newCurrent.next = current.next;
        current.next = current.next.next;
        current = current.next;
        newCurrent = newCurrent.next;
    }
    return newFakeHead.next;
};
// @lc code=end

function assertEqualsButNotSame(list1, list2) {
    while(true) {
        if(list1==null&&list2==null) {
            return true;
        }
        else if(list1==null||list2==null) {
            return false;
        }
        else if(list1==list2) {
            return false;
        }
        else if(list1.val!==list2.val) {
            return false;
        }
        else if(!(list1.random==null&&list2.random==null)
            &&((list1.random==null||list2.random==null)||(list1==list2))) {
            return false;
        }
        list1 = list1.next;
        list2 = list2.next;
    }
}

function testCopyRandomList() {
    const node1 = new Node(1,null,null);
    const node2 = new Node(2,null,null);
    node1.next = node2;
    node1.random = node2;
    node2.random = node2;
    const truth = copyRandomList(node1);
    let truthCur = truth;
    let expectedCur = node1;
    assert(assertEqualsButNotSame(truthCur, expectedCur));
}

if(module.parent==null) {
    testCopyRandomList()
}