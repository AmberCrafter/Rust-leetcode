pub struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m,n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut marked_table = vec![vec![0; n]; m];
        
        for i in 0..m {
            if obstacle_grid[i][0]!=1 { marked_table[i][0] = 1} else {break;};
        }

        for i in 0..n {
            if obstacle_grid[0][i]!=1 { marked_table[0][i] = 1} else {break;};
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j]!=1 {
                    marked_table[i][j] = marked_table[i-1][j] + marked_table[i][j-1];
                }
            }
        }
        marked_table[m-1][n-1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![
            vec![0,0,0],
            vec![0,1,0],
            vec![0,0,0]
        ];
        let except = 2;
        let result = Solution::unique_paths_with_obstacles(inputs);
        assert_eq!(except, result);
    }

    #[test]
    fn case2() {
        let inputs = vec![
            vec![0,1],
            vec![0,0]
        ];
        let except = 1;
        let result = Solution::unique_paths_with_obstacles(inputs);
        assert_eq!(except, result);
    }
}