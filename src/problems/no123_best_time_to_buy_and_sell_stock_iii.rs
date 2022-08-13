pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length = prices.len();
        if length==2 {
            return if prices[1]>prices[0] {prices[1]-prices[0]} else {0};
        }

        let mut dp = vec![vec![0; length]; length];
        for i in 0..length-1 {
            for j in i+1..length {
                dp[i][j] = prices[j] - prices[i];
                if dp[i][j]>dp[i][0] {dp[i][0]=dp[i][j];}
            }
        }
        for j in 1..length {
            dp[length-1][j-1] = (0..length-1).map(|i| dp[i][j]).max().unwrap();
        }



        let mut res = 0;
        for j in 0..length-1 {
            for i in j+1..length-1 {
                let tmp = dp[length-1][j] + dp[i][0];
                if tmp>res {res = tmp;}
            }
        }

        for val in dp {
            println!("{:?}", val);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![3,3,5,0,0,3,1,4];
        let except = 6;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![1,2,3,4,5];
        let except = 4;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = vec![7,6,4,3,1];
        let except = 0;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = vec![6,1,3,2,4,7];
        let except = 7;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case5() {
        let inputs = vec![1,2];
        let except = 1;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case6() {
        let inputs = vec![1,4,2];
        let except = 3;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }
}