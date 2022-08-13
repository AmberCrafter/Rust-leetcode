pub struct Solution {}

#[derive(Debug, PartialEq)]
enum Chess {
    Empty,
    Queen,
}

#[derive(Debug)]
struct CheeseBoard {
    size: usize,
    board: Vec<Vec<Chess>>,
}

impl CheeseBoard {
    pub fn new(n: i32) -> Self {
        Self {
            size: n as usize,
            board: (0..n)
                .map(|_| (0..n).map(|_| Chess::Empty).collect())
                .collect(),
        }
    }

    fn test_row(&self, row: usize) -> bool {
        (0..self.size)
            .find(|&col| self.board[row][col] == Chess::Queen)
            .is_none()
    }

    fn test_col(&self, col: usize) -> bool {
        (0..self.size)
            .find(|&row| self.board[row][col] == Chess::Queen)
            .is_none()
    }

    fn test_leftoblique(&self, row: usize, col: usize) -> bool {
        // move to left top
        // test to right down
        let (mut x, mut y) = (row - usize::min(row, col), col - usize::min(row, col));
        while x < self.size && y < self.size {
            if self.board[x][y] == Chess::Queen {
                return false;
            }
            x += 1;
            y += 1;
        }
        true
    }

    fn test_rightoblique(&self, row: usize, col: usize) -> bool {
        // move to right top
        // test to left down
        let (mut x, mut y) = (
            row - usize::min(row, self.size - col - 1),
            col + usize::min(row, self.size - col - 1),
        );
        while x < self.size && y < self.size {
            if self.board[x][y] == Chess::Queen {
                return false;
            }
            if y == 0 {
                break;
            }
            x += 1;
            y -= 1;
        }
        true
    }

    fn is_valid(&self, row: usize, col: usize) -> bool {
        self.test_row(row)
            && self.test_col(col)
            && self.test_leftoblique(row, col)
            && self.test_rightoblique(row, col)
    }

    fn put_queen(&mut self, row: usize, col: usize) -> bool {
        if self.is_valid(row, col) {
            self.board[row][col] = Chess::Queen;
            return true;
        }
        false
    }

    fn take_queen(&mut self, row: usize, col: usize) {
        self.board[row][col] = Chess::Empty;
    }
}

impl Solution {
    fn dfs_player(result: &mut i32, board: &mut CheeseBoard, row: usize, col: usize) {
        if row >= board.size {
            *result += 1;
            return ();
        }
        if board.put_queen(row, col) {
            Solution::dfs_player(result, board, row + 1, 0);
            board.take_queen(row, col);
        }
        if col < board.size - 1 {
            Solution::dfs_player(result, board, row, col + 1);
        }
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut result: i32 = 0;
        let mut board = CheeseBoard::new(n);
        Solution::dfs_player(&mut result, &mut board, 0, 0);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = 4;
        let expected = 2;
        let result = Solution::total_n_queens(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = 1;
        let expected = 1;
        let result = Solution::total_n_queens(inputs);

        assert_eq!(expected, result);
    }
}
