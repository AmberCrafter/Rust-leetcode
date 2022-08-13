pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // ref. https://leetcode.com/problems/unique-paths/discuss/240806/Rust-solution
        // Because the robot only walk down and right, the algorithm need to care about
        // the times which the robot come from the top or left box.

        let n = n as usize;
        let m = m as usize;
        let mut paths = vec![vec![0; n]; m];

        for i in 0..m {
            paths[i][0] = 1;
        }
        for i in 0..n {
            paths[0][i] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                paths[i][j] = paths[i - 1][j] + paths[i][j - 1];
            }
        }
        return paths[m - 1][n - 1] as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let inputs = (3, 7);
        let expect = 28;
        let result = Solution::unique_paths(inputs.0, inputs.1);
        assert_eq!(result, expect);
    }

    #[test]
    fn case2() {
        let inputs = (3, 2);
        let expect = 3;
        let result = Solution::unique_paths(inputs.0, inputs.1);
        assert_eq!(result, expect);
    }
}
