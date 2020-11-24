const assert = require('assert');

// Definition for a Node.
function Node(val, next) {
    this.val = val;
    this.next = next;
};

/*
 * @lc app=leetcode.cn id=708 lang=javascript
 *
 * [708] 循环有序列表的插入
 */

// @lc code=start
/**
 * @param {Node} head
 * @param {number} insertVal
 * @return {Node}
 */
var insert = function(head, insertVal) {
    if(!head) {
      const newNode = new Node(insertVal, null);
      newNode.next = newNode;
      return newNode;
    }
    let current = head;
    let count = 0;
    let insertPos = -1;
    while (true) {
      const next = current.next;
      if (next.val >= insertVal && current.val <= insertVal) {
        const newNode = new Node(insertVal, null);
        current.next = newNode;
        newNode.next = next;
        return head;
      }
      if (next.val < current.val) {
        insertPos = count;
      }
      if (Object.is(next, head)) {
        break;
      }
      current = next;
      count += 1;
    }
    if (insertPos === -1) {
      const next = current.next;
      const newNode = new Node(insertVal, null);
      current.next = newNode;
      newNode.next = next;
    } else {
      let current = head;
      for(let i=0;i<insertPos; i++) {
        current = current.next;
      }
      const next = current.next;
      const newNode = new Node(insertVal, null);
      current.next = newNode;
      newNode.next = next;
    }
    return head;
};
// @lc code=end

const loopEqual = (l, r) => {
  const lHead = l;
  const rHead = r;
  while (true) {
    if (Object.is(l, r)) {
      return true;
    } else if (!l || !r || r.val !== l.val) {
      return false;
    }
    r = r.next;
    l = l.next;
    if (Object.is(l, lHead) || Object.is(r, rHead)) {
      if(Object.is(l, lHead) && Object.is(r, rHead)) {
        return true;
      } else {
        return false;
      }
    }
  }
}

const test1 = () => {
  const loop = new Node(3, new Node(4, new Node(1, null)));
  loop.next.next.next = loop;
  const expect = new Node(3, new Node(4, new Node(1, new Node(2, null))));
  expect.next.next.next.next = expect;
  assert(loopEqual(insert(loop, 2), expect))
}

const test2 = () => {
  const loop = null;
  const expect = new Node(2, null);
  expect.next = expect;
  assert(loopEqual(insert(loop, 2), expect))
}

const test3 = () => {
  const loop = new Node(3, new Node(3, new Node(3, null)));
  loop.next.next.next = loop;
  const expect = new Node(3, new Node(3, new Node(3, new Node(0, null))));
  expect.next.next.next.next = expect;
  assert(loopEqual(insert(loop, 0), expect))
}

test1();
test2();
test3();