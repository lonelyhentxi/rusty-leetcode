/**
 * Definition for read4()
 * 
 * @param {character[]} buf Destination buffer
 * @return {number} The number of actual characters read
 * read4 = function(buf) {
 *     ...
 * };
 */

/*
 * @lc app=leetcode.cn id=157 lang=javascript
 *
 * [157] 用 Read4 读取 N 个字符
 */


/**
 * @param {function} read4()
 * @return {function}
 */
var solution = function(read4) {
    /**
     * @param {character[]} buf Destination buffer
     * @param {number} n Number of characters to read
     * @return {number} The number of actual characters read
     */
    return function(buf, n) {
        let remain = n;
        while(remain>0) {
            const temp = [];
            const count = read4(temp);
            const toAdd = Math.min(count,remain);
            buf.push(...temp.slice(0, toAdd));
            remain -= toAdd;
            if(toAdd<4) {
                break;
            }
        }
    };
};

// @lc code=end