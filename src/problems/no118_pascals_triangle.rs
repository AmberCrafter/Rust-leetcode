pub struct Solution {}
impl Solution {
    // best: https://leetcode.com/problems/pascals-triangle/discuss/2242371/Rust-solution-(rule)
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        match num_rows {
            0 => {},
            1 => res.push(vec![1]),
            2 => {res.push(vec![1]); res.push(vec![1,1]);},
            _ => {
                res = Solution::generate(2);
                for i in 2..num_rows {
                    let mut tmp = vec![1];
                    let mut prev_vec = res.last().unwrap();
                    for j in 0..prev_vec.len()-1 {
                        tmp.push(prev_vec[j]+prev_vec[j+1]);
                    }
                    tmp.push(1);
                    res.push(tmp);
                }
            }
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = 5;
        let except = vec![
            vec![1],
            vec![1,1],
            vec![1,2,1],
            vec![1,3,3,1],
            vec![1,4,6,4,1]
        ];
        let output = Solution::generate(inputs);
        assert_eq!(except, output);
    }
}