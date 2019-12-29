const assert = require('assert');

/*
 * @lc app=leetcode.cn id=191 lang=javascript
 *
 * [191] 位1的个数
 */

// @lc code=start
/**
 * @param {number} n - a positive integer
 * @return {number}
 */
var hammingWeight = function(n) {
    let sum = 0;
    while(n!=0) {
        sum ++;
        n &= (n-1);
    }
    return sum;
};
// @lc code=end

function testHammingWeight() {
    assert.equal(hammingWeight(parseInt('1011',2)),3);
    assert.equal(hammingWeight(parseInt('10000000',2)),1);
    assert.equal(hammingWeight(parseInt('11111111111111111111111111111101',2)),31);
}

if (module.parent==null) {
    testHammingWeight();
}