/*
 * @lc app=leetcode.cn id=348 lang=rust
 *
 * [348] 判定井字棋胜负
 */

// @lc code=start
#[derive(Debug)]
struct TicTacToe {
    rows: Vec<Option<i32>>,
    cols: Vec<Option<i32>>,
    crosses: (Option<i32>, Option<i32>),
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TicTacToe {
    /** Initialize your data structure here. */
    fn new(n: i32) -> Self {
        TicTacToe {
            rows: vec![Some(0); n as usize],
            cols: vec![Some(0); n as usize],
            crosses: (Some(0), Some(0)),
            size: n as usize,
        }
    }

    /** Player {player} makes a move at ({row}, {col}).
    @param row The row of the board.
    @param col The column of the board.
    @param player The player, can be either 1 or 2.
    @return The current winning condition, can be either:
            0: No one wins.
            1: Player 1 wins.
            2: Player 2 wins. */
    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let size = self.size;
        if (TicTacToe::update_line(&mut self.rows[row as usize], player, size) == 0)
            && (TicTacToe::update_line(&mut self.cols[col as usize], player, size) == 0)
            && !(row == col && TicTacToe::update_line(&mut self.crosses.0, player, size) != 0)
            && !(row == (size as i32 - 1 - col) && TicTacToe::update_line(&mut self.crosses.1, player, size) != 0)
        {
            0
        } else {
            player
        }
    }

    fn update_line(line: &mut Option<i32>, player: i32, size: usize) -> i32 {
        let mut line_winner = 0;
        let next_line = match *line {
            Some(count) => {
                if player == 1 && count >= 0 {
                    let next_count = count + 1;
                    if next_count as usize == size {
                        line_winner = 1;
                    }
                    Some(next_count)
                } else if player == 2 && count <= 0 {
                    let next_count = count - 1;
                    if next_count == 0 - size as i32 {
                        line_winner = 2;
                    }
                    Some(next_count)
                } else {
                    None
                }
            }
            None => None,
        };
        *line = next_line;
        line_winner
    }
}
// @lc code=end


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tic_tac_toe() {
        let mut toe = TicTacToe::new(3);
        assert_eq!(toe.make_a_move(0, 0, 1), 0);
        assert_eq!(toe.make_a_move(0, 2, 2), 0);
        assert_eq!(toe.make_a_move(2, 2, 1), 0);
        assert_eq!(toe.make_a_move(1, 1, 2), 0);
        assert_eq!(toe.make_a_move(2, 0, 1), 0);
        assert_eq!(toe.make_a_move(1, 0, 2), 0);
        assert_eq!(toe.make_a_move(2, 1, 1), 1);
    }
}