pub struct Solution {}
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let nrow = matrix.len();
        let ncol = matrix[0].len();

        // check row_0 or col_0 have 0?
        let is_row_has0 = (0..nrow).any(|i| matrix[i][0] == 0);
        let is_col_has0 = (0..ncol).any(|j| matrix[0][j] == 0);

        // toggle 0 postion
        for i in 1..nrow {
            for j in 1..ncol {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // set position to 0
        for i in 1..nrow {
            for j in 1..ncol {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // set row_0 or col_0 to 0
        if is_row_has0 {
            (0..nrow).for_each(|i| matrix[i][0] = 0);
        }
        if is_col_has0 {
            (0..ncol).for_each(|j| matrix[0][j] = 0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let except = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let output = Solution::set_zeroes(&mut inputs);
        assert_eq!(except, inputs);
    }

    #[test]
    fn case2() {
        let mut inputs = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let except = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        let output = Solution::set_zeroes(&mut inputs);
        assert_eq!(except, inputs);
    }
}
