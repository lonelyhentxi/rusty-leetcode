const assert = require('assert');

// Definition for a Node.
function Node(val, children) {
  this.val = val;
  this.children = children;
};

/*
 * @lc app=leetcode.cn id=428 lang=javascript
 *
 * [428] 序列化和反序列化 N 叉树
 */

// @lc code=start
class Codec {
    /** 
     * @param {Node} root
     * @return {string}
     */
    // Encodes a tree to a single string.
    serialize = function(root) {
      return JSON.stringify(root);
    };
	
    /** 
     * @param {string} data 
     * @return {Node}
     */
    // Decodes your encoded data to tree.
    deserialize = function(data) {
      return JSON.parse(data);
    };
}

// Your Codec object will be instantiated and called as such:
// Codec codec = new Codec();
// codec.deserialize(codec.serialize(root));
// @lc code=end