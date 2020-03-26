/**
 * Definition for read4()
 * 
 * @param {character[]} buf Destination buffer
 * @return {number} The number of characters read
 * read4 = function(buf) {
 *     ...
 * };
 */

 /*
 * @lc app=leetcode.cn id=158 lang=javascript
 *
 * [158] 用 Read4 读取 N 个字符 II
 */

/**
 * @param {function} read4()
 * @return {function}
 */
var solution = function(read4) {
    const noneUsed = [];
    /**
     * @param {character[]} buf Destination buffer
     * @param {number} n Number of characters to read
     * @return {number} The number of actual characters read
     */
    return function(buf, n) {
        let remain = Math.max(0, n - noneUsed.length);
        while(remain>0) {
            const temp = [];
            const count = read4(temp);
            const toAdd = Math.min(count, remain);
            remain -= toAdd;
            noneUsed.push(...temp);
            if(toAdd<4) {
                break;
            }
        }
        buf.push(...noneUsed.splice(0, n - remain));
        return n-remain;
    };
};

// @lc code=end