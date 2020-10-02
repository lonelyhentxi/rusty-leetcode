// Definition for a Node.
function Node(val, children) {
  this.val = val;
  this.children = children;
};


// Definition for a binary tree node.
function TreeNode(val) {
  this.val = val;
  this.left = this.right = null;
}



/*
 * @lc app=leetcode.cn id=431 lang=javascript
 *
 * [431] 将 N 叉树编码为二叉树
 */

// @lc code=start
class Codec {
    /** 
     * @param {Node} root
     * @return {TreeNode}
     */
    // Encodes an n-ary tree to a binary tree.
    encode = function(root) {
      const roots = root ? [root] : [];
      return this.encodeRecursive(roots, 0);
    };

    encodeRecursive = function(siblings, index) {
      if (!Array.isArray(siblings) || index >= siblings.length) {
        return null;
      }
      const current = siblings[index];
      const newNode = new TreeNode(current.val);
      const children = current.children;
      newNode.right = this.encodeRecursive(siblings, index + 1);
      newNode.left = this.encodeRecursive(children, 0);
      return newNode;
    }
	
    /** 
     * @param {TreeNode} root 
     * @return {Node}
     */
    // Decodes your binary tree to an n-ary tree.
    decode = function(root) {
      const roots = this.decodeRecursive(root);
      if(roots.length <= 0) {
        return null;
      } else {
        return roots[0];
      }
    };

    decodeRecursive(root) {
      if (!root) {
        return [];
      }
      const children = this.decodeRecursive(root.left);
      const newNode = new Node(root.val, children);
      const siblings = [newNode];
      siblings.push(...this.decodeRecursive(root.right));
      return siblings;
    }
}

/*
* Your Codec object will be instantiated and called as such:
* codec = Codec()
* codec.decode(codec.encode(root))
*/
// @lc code=end

