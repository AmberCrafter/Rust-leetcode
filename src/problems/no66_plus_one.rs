pub struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut c = 1;
        let mut idx = digits.len() as i32 - 1;
        while (c > 0) && (idx >= 0) {
            let tmp = digits[idx as usize] + c;
            c = tmp / 10;
            digits[idx as usize] = tmp % 10;
            idx -= 1;
        }
        if c>0 {digits.insert(0, c)};
        digits
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1,2,3];
        let except = vec![1,2,4];
        let result = Solution::plus_one(inputs);
        assert_eq!(except, result);
    }

    #[test]
    fn case2() {
        let inputs = vec![4,3,2,1];
        let except = vec![4,3,2,2];
        let result = Solution::plus_one(inputs);
        assert_eq!(except, result);
    }

    #[test]
    fn case3() {
        let inputs = vec![9];
        let except = vec![1,0];
        let result = Solution::plus_one(inputs);
        assert_eq!(except, result);
    }
}
