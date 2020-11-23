#include <utility>
#include <vector>
#include <cassert>

namespace pre {
    using namespace std;

    constexpr int overflow = 2147483647;

    class ArrayReader {
        vector<int> items;
    public:
        explicit ArrayReader(vector<int> contents) : items{std::move(contents)} {}

        [[nodiscard]]
        int get(int index) const {
            if (index >= items.size()) {
                return overflow;
            } else {
                return items[index];
            }
        }
    };
}

using pre::ArrayReader;

/*
 * @lc app=leetcode.cn id=702 lang=cpp
 *
 * [702] 搜索长度未知的有序数组
 */

// @lc code=start

#define let const auto

constexpr int overflow = 2147483647;

class Solution {
public:
    int search(const ArrayReader &reader, int target) {
        auto start = 0;
        auto end = 1 << 30;
        auto mid = end;
        while (end > start) {
            let v = reader.get(mid);
            if (v == overflow) {
                while (reader.get(mid) == overflow) {
                    end = mid;
                    mid = mid / 2;
                }
                mid = (start + end) / 2;
            } else if (v == target) {
                return mid;
            } else if (v > target) {
                end = mid - 1;
                mid = (start + end) / 2;
            } else {
                start = mid + 1;
                mid = (start + end) / 2;
            }
        }
        return reader.get(mid) == target ? mid : -1;
    }
};
// @lc code=end

int main() {
    auto s = Solution{};
    assert(s.search(ArrayReader{{ -1,0,3,5,9,12 }}, 9) == 4);
    assert(s.search(ArrayReader{{ -1,0,3,5,9,12 }}, 2) == -1);
    assert(s.search(ArrayReader{{ -1,0,3,5,9,12 }}, 12) == 5);
}