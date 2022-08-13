pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut tmp = 0;
        let mut res = 0;
        for i in 0..prices.len()-1 {
            tmp += prices[i+1]-prices[i];
            if tmp<0 {tmp=0;} else if tmp>res {
                res = tmp;
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
        let inputs = vec![7,1,5,3,6,4];
        let except = 5;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }
}