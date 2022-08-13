pub struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut nrow = matrix.len();
        let mut ncol = matrix[0].len();

        // check lower and upper
        if (target < matrix[0][0]) || (target > matrix[nrow - 1][ncol - 1]) {
            return false;
        }

        for i in (0..nrow).rev() {
            if matrix[i][0] == target {
                return true;
            }
            if matrix[i][0] < target {
                nrow = i;
                break;
            }
        }

        for j in (0..ncol).rev() {
            if matrix[nrow][j] == target {
                return true;
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
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
        );
        let except = true;
        let output = Solution::search_matrix(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13,
        );
        let except = false;
        let output = Solution::search_matrix(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            11,
        );
        let except = true;
        let output = Solution::search_matrix(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
