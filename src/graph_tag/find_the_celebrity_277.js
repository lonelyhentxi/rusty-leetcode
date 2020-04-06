/*
 * @lc app=leetcode.cn id=277 lang=rust
 *
 * [277]  搜寻名人
 */
// @lc code=start
/**
 * Definition for knows()
 * 
 * @param {integer} person a
 * @param {integer} person b
 * @return {boolean} whether a knows b
 * knows = function(a, b) {
 *     ...
 * };
 */
/**
 * @param {function} knows()
 * @return {function}
 */
var solution = function (knows) {
    /**
     * @param {integer} n Total people
     * @return {integer} The celebrity
     */
    return function (n) {
        if (n === 0) {
            return -1;
        }
        else if (n === 1) {
            return 0;
        }
        let maybe = 0;
        for(let i=0; i < n; i++) {
            if(knows(maybe,i)) {
                maybe = i;
            }
        }
        for(let i=0; i< n;i++) {
            if(i!==maybe) {
                if(!knows(i, maybe) || knows(maybe,i)) {
                    return -1;
                }
            }
        }
        return maybe;
    };
};
// @lc code=end

const assert = require('assert');

const mat = [[1, 1, 0], [0, 1, 0], [1, 1, 1]];
const knows = (a, b) => {
    return mat[a][b];
}

assert.equal(solution(knows)(3), 1);