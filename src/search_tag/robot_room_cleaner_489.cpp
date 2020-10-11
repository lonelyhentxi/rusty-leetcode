#include <cassert>
#include <stack>
#include <tuple>
#include <array>
#include <unordered_set>
#include <cstdint>
#include <vector>

#define let const auto

namespace test {
    using namespace std;

    using position = tuple<int32_t, int32_t>;
    constexpr int32_t DIRECTION_SIZE = 4;
    constexpr array<position, DIRECTION_SIZE> DIRECTIONS = array<position, DIRECTION_SIZE>{position{0, -1},
                                                                                           position{1, 0},
                                                                                           position{0, 1},
                                                                                           position{-1, 0}};

    class Robot {
        vector<vector<int32_t>> room;
        int32_t forward = 0;
        int32_t x;
        int32_t y;
        int32_t rows;
        int32_t cols;

    public:
        Robot(vector<vector<int32_t>> room, int32_t x, int32_t y) : room{room}, x{x}, y{y} {
            rows = room.size();
            cols = rows == 0 ? 0 : room[0].size();
        }

        bool move() {
            let d = DIRECTIONS[forward];
            let dx = get<0>(d);
            let dy = get<1>(d);
            let nx = x + dx;
            let ny = y + dy;
            if (nx < 0 || ny < 0 || nx >= cols || ny >= rows || room[ny][nx] == 0) {
                return false;
            } else {
                x = nx;
                y = ny;
                return true;
            }
        }

        void turnLeft() {
            forward = (forward + DIRECTION_SIZE - 1) % DIRECTION_SIZE;
        }

        void turnRight() {
            forward = (forward + 1) % DIRECTION_SIZE;
        }

        void clean() {
            room[y][x] += 1;
        }

        bool isAllClean() {
            for (auto i = 0; i < rows; i++) {
                for (auto j = 0; j < cols; j++) {
                    if (room[i][j] == 1) {
                        return false;
                    }
                }
            }
            return true;
        }

        bool isNoDupClean() {
            for (auto i = 0; i < rows; i++) {
                for (auto j = 0; j < cols; j++) {
                    if (room[i][j] > 2) {
                        return false;
                    }
                }
            }
            return true;
        }
    };
}

using test::Robot;

/*
 * @lc app=leetcode.cn id=489 lang=cpp
 *
 * [489] 扫地机器人
 */

// @lc code=start
#include <stack>
#include <tuple>
#include <array>
#include <unordered_set>
#include <cstdint>
#include <string>
#include <optional>

#define let const auto

using namespace std;

using direction = int32_t;
using pos_unit = int32_t;
using position = tuple<pos_unit, pos_unit>;
using pos_dir = tuple<position, direction>;
constexpr int32_t DIRECTION_SIZE = 4;
constexpr array<position, DIRECTION_SIZE> DIRECTIONS = array<position, DIRECTION_SIZE>{position{0, -1}, position{1, 0},
                                                                                       position{0, 1}, position{-1, 0}};

class Agent {
public:
    unordered_set<string> clean_memory;
    stack<direction> movements;
    stack<pos_dir> targets;

    direction forward;
    pos_unit x;
    pos_unit y;

    Robot &robot;

    explicit Agent(Robot &robot) : clean_memory{}, movements{}, targets{}, forward{0}, x{0}, y{0}, robot{robot} {
        clean();
        prepareTargets();
    }

    static string hashPosition(const position &p) {
        let px = get<0>(p);
        let py = get<1>(p);
        auto s = to_string(px);
        return s + ':' + to_string(py);
    }

    bool cleaned(pos_unit tx, pos_unit ty) const {
        return clean_memory.find(hashPosition({tx, ty})) != clean_memory.cend();
    }

    bool cleaned() const {
        return cleaned(x, y);
    }

    bool tryMove(bool goForward = true) {
        let movable = robot.move();
        if (movable) {
            if(goForward) {
                movements.push(forward);
            }
            let d = DIRECTIONS[forward];
            let dx = get<0>(d);
            let dy = get<1>(d);
            x = dx + x;
            y = dy + y;
        }
        return movable;
    }

    void prepareTargets() {
        for (auto i = DIRECTION_SIZE - 1; i >= 0; i--) {
            let nf = (i + forward) % DIRECTION_SIZE;
            let d = DIRECTIONS[nf];
            let dx = get<0>(d);
            let dy = get<1>(d);
            let nx = dx + x;
            let ny = dy + y;
            if (!cleaned(nx, ny)) {
                targets.push({{x, y}, nf});
            }
        }
    }

    void clean() {
        robot.clean();
        clean_memory.insert(hashPosition({x, y}));
    }

    bool finished() const {
        return targets.empty();
    }

    void backToTargetEntry(const pos_dir &target) {
        let pos = get<0>(target);
        let tx = get<0>(pos);
        let ty = get<1>(pos);
        while (tx != x || ty != y) {
            let f = movements.top();
            movements.pop();
            let b = (f + 2) % DIRECTION_SIZE;
            turnTo(b);
            tryMove(false);
        }
    }

    void turn() {
        forward = (forward + 1) % DIRECTION_SIZE;
        robot.turnRight();
    }

    void turnTo(direction d) {
        while (forward != d) {
            turn();
        }
    }

    pos_dir popOneTarget() {
        auto res = targets.top();
        targets.pop();
        return res;
    }
};

class Solution {
public:
    void cleanRoom(Robot &robot) {
        auto agent = Agent(robot);
        while (!agent.finished()) {
            let target = agent.popOneTarget();
            agent.backToTargetEntry(target);
            let tf = get<1>(target);
            agent.turnTo(tf);
            let movable = agent.tryMove();
            if (movable && !agent.cleaned()) {
                agent.clean();
                agent.prepareTargets();
            }
        }
    }
};
// @lc code=end

int main() {
    {
        let room = vector<vector<int32_t>>{
                {1, 1, 1, 1, 1, 0, 1, 1},
                {1, 1, 1, 1, 1, 0, 1, 1},
                {1, 0, 1, 1, 1, 1, 1, 1},
                {0, 0, 0, 1, 0, 0, 0, 0},
                {1, 1, 1, 1, 1, 1, 1, 1}
        };
        auto robot = Robot(room, 3, 1);
        Solution{}.cleanRoom(robot);
        assert(robot.isAllClean());
        assert(robot.isNoDupClean());
    }
}