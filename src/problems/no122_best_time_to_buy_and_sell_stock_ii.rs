pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profix = 0;
        for i in 0..prices.len()-1 {
            let diff = prices[i+1] - prices[i];
            profix += if diff>0 {diff} else {0};
        }
        profix
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![7,1,5,3,6,4];
        let except = Solution::max_profit(inputs);
        let output = 7;
        assert_eq!(except, output);
    }
}