pub struct Solution {}
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut grid = grid;

        for i in 1..m {
            grid[i][0] += grid[i - 1][0];
        }

        for i in 1..n {
            grid[0][i] += grid[0][i - 1];
        }

        for i in 1..m {
            for j in 1..n {
                grid[i][j] += grid[i - 1][j].min(grid[i][j - 1]);
            }
        }
        grid[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        let except = 7;
        let result = Solution::min_path_sum(inputs);
        assert_eq!(except, result);
    }

    #[test]
    fn case2() {
        let inputs = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let except = 12;
        let result = Solution::min_path_sum(inputs);
        assert_eq!(except, result);
    }
}
