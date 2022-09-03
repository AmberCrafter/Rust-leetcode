pub struct Solution {}
impl Solution {
    // ref: https://www.cnblogs.com/grandyang/p/4281975.html
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // // 2D - dynamic programming solution
        // // row [i]: day
        // // col [j]: number of transation times
        // // local table: the best profit of selling today with bought yesterday(global table) or bought before yesterday(local table)
        // // global table: the current best profit

        // let num = prices.len(); // number of days
        // let ttimes = 2; // transation times
        // let mut lt = vec![vec![0; ttimes+1]; num];
        // let mut gt = vec![vec![0; ttimes+1]; num];

        // for i in 1..num {
        //     for j in 1..ttimes+1 {
        //         let diff = prices[i] - prices[i-1];
        //         lt[i][j] = ((gt[i-1][j-1] + diff.max(0))).max(lt[i-1][j] + diff); // choosing 0 menas no bought stock yesterday or before yesterday
        //         gt[i][j] = gt[i-1][j].max(lt[i][j]);
        //     }
        // }
        // gt[num-1][ttimes]


        // Because we only need the informaiton of previous day
        let num = prices.len(); // number of days
        let ttimes = 2; // transation times
        let mut lt = vec![0; ttimes+1];
        let mut gt = vec![0; ttimes+1];

        for i in 1..num {
            for j in (1..ttimes+1).rev() {
                let diff = prices[i] - prices[i-1];
                lt[j] = ((gt[j-1] + diff.max(0))).max(lt[j] + diff); // choosing 0 menas no bought stock yesterday or before yesterday
                gt[j] = gt[j].max(lt[j]);
            }
        }
        gt[ttimes]
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

    #[test]
    fn case7() {
        let inputs = vec![1,2,4,2,5,7,2,4,9,0];
        let except = 13;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }
}