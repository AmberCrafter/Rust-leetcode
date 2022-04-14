pub struct Solution {}

impl Solution {
    pub fn next_step(res: &mut i32, n: i32, m: i32, x: i32 ,y: i32) {
        if x==n-1 && y==m-1 { *res+=1; return}
        if x<n-1 {
            Solution::next_step(res, n, m, x+1, y);
        }
        if y<m-1 {
            Solution::next_step(res, n, m, x, y+1);
        }
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut res = 0;
        Solution::next_step(&mut res, n, m, 0, 0);
        res
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