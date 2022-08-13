pub struct Solution {}
impl Solution {
    // backtracking
    fn next(board: &mut Vec<Vec<char>>, word: &[u8], i: usize, j: usize) -> bool {
        if word.len() == 0 {
            return true;
        };
        let curr = board[i][j];
        board[i][j] = '-';
        let status = 
            // up
            if i>0 {
                if board[i-1][j] as u8 == word[0] {
                    Solution::next(board, &word[1..], i-1, j)
                } else {false}
            } else {false} || 
            // down
            if i<board.len()-1 { 
                if board[i+1][j] as u8 == word[0] {
                    Solution::next(board, &word[1..], i+1, j)
                } else {false}
            } else {false} || 
            // left
            if j>0 { 
                if board[i][j-1] as u8 == word[0] {
                    Solution::next(board, &word[1..], i, j-1)
                } else {false}
            } else {false} || 
            // right
            if j<board[0].len()-1 { 
                if board[i][j+1] as u8 == word[0] {
                    Solution::next(board, &word[1..], i, j+1)
                } else {false}
            } else {false};
        board[i][j] = curr;
        status
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let word = word.as_bytes();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] as u8 == word[0] {
                    if Solution::next(&mut board, &word[1..], i, j) {
                        return true;
                    };
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_string(),
        );
        let except = true;
        let output = Solution::exist(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
