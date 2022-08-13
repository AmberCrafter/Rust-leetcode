pub struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![1; (row_index+1) as usize];
        for i in 0..row_index {
            for j in (1..(i+1) as usize ).rev() {
                res[j] = res[j] + res[j-1];
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
        let inputs = 3;
        let except = vec![1,3,3,1];
        let output = Solution::get_row(inputs);
        assert_eq!(except, output);
    }
}