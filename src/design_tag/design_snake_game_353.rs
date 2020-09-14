/*
 * @lc app=leetcode.cn id=353 lang=rust
 *
 * [353] 贪吃蛇
 */

// @lc code=start
use std::collections::VecDeque;
use std::collections::HashSet;

const UP: (&str, isize, isize) = ("U", 0, -1);
const DOWN: (&str, isize, isize) = ("D", 0, 1);
const LEFT: (&str, isize, isize) = ("L", -1, 0);
const RIGHT: (&str, isize, isize) = ("R", 1, 0);

const DIRECTIONS: [(&str, isize, isize); 4] = [UP, DOWN, LEFT, RIGHT];


#[derive(Debug)]
struct SnakeGame {
    snake: VecDeque<(isize, isize)>,
    body: HashSet<(isize, isize)>,
    foods: Vec<(isize, isize)>,
    width: isize,
    height: isize
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnakeGame {
    /** Initialize your data structure here.
    @param width - screen width
    @param height - screen height
    @param food - A list of food positions
    E.g food = [[1,1], [1,0]] means the first food is positioned at [1,1], the second is at [1,0]. */
    pub fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        SnakeGame {
            snake: {
                let mut res = VecDeque::new();
                res.push_back((0, 0));
                res
            },
            body: HashSet::new(),
            foods: food
                .into_iter()
                .map(|f| (f[1] as isize, f[0] as isize))
                .rev()
                .collect(),
            width: width as isize,
            height: height as isize
        }
    }

    /** Moves the snake.
    @param direction - 'U' = Up, 'L' = Left, 'R' = Right, 'D' = Down
    @return The game's score after the move. Return -1 if game over.
    Game over when snake crosses the screen boundary or bites its body. */
    pub fn make_a_move(&mut self, direction: String) -> i32 {
        let head = self.head();
        self.body.insert(head);
        let mut next_head: (isize, isize) = (0, 0);
        for d in &DIRECTIONS {
            if direction == d.0 {
                next_head = (head.0 + d.1, head.1 + d.2);
                break;
            }
        }
        let mut will_eat = false;
        if next_head.0 < 0 || next_head.1 < 0 || next_head.0 >= self.width || next_head.1 >= self.height {
            return -1;
        } else if let Some(f) = self.foods.last()  {
            if f.0 == next_head.0 && f.1 == next_head.1 {
                will_eat = true;
            }
        }
        match self.snake.pop_back() {
            Some(tail_end) => {
                self.body.remove(&tail_end);
                if will_eat {
                    self.foods.pop();
                    self.snake.push_back(tail_end);
                    self.body.insert(tail_end);
                }
            },
            None => { return -1 }
        }
        if self.body.contains(&next_head) {
            return -1;
        }
        self.snake.push_front(next_head);
        (self.snake.len() - 1) as i32
    }
    fn head(&self) -> (isize, isize) {
        self.snake.get(0).unwrap().clone()
    }
}

// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snake_game() {
        let mut snake_game = SnakeGame::new(3, 2, vec![vec![1, 2], vec![0, 1]]);
        let directions = ["R", "D", "R", "U", "L", "U"];
        let result = [0, 0, 1, 1, 2, -1];
        for i in 0..directions.len() {
            assert_eq!(snake_game.make_a_move(String::from(directions[i])), result[i]);
        }
    }
}