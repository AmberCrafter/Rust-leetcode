pub struct Solution {}
impl Solution {
    // best: https://leetcode.com/problems/triangle/discuss/727074/Rust-0ms-DP
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut res = vec![0; triangle.len()];
        for layer in triangle.iter() {
            for (i, ele) in layer.iter().enumerate() {
                res[i] += ele;
            }
            // extend
            if layer.len()!=triangle.len() {
                for i in (0..layer.len()).rev() {
                    res[i+1] = if i==layer.len()-1 {
                        res[i]
                    } else {
                        res[i].min(res[i+1])
                    }
                }
            }
        }
        *res.iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![
            vec![2],
            vec![3,4],
            vec![6,5,7],
            vec![4,1,8,3]
        ];
        let except = 11;
        let output = Solution::minimum_total(inputs);
        assert_eq!(except, output);
    }
}