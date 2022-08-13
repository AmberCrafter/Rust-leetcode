pub struct Solution {}
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // ref. https://leetcode.com/problems/unique-binary-search-trees/discuss/1121661/Rust%3A-dp-solution
        //
        // we make the in-order input array as [a,b,c,...]
        // imaging that insert a new value, then the total different valid pattern numbers can get from
        // left hand side case nums times with right hand side case nums which seperate by insert value.
        //
        // n=0
        // [[]]
        // total: 1 (init value)
        // n=1
        //  [[1]]
        // total: 1
        // n=2
        //    1
        //   ^ ^
        //  [[1,2], [2,1]]
        // total: 2
        // n=3
        //    1 2
        //   ^ ^ ^
        //  [[O,1,2], [1,O,2], [1,2,O]]
        //  explanation:
        //    take [O,1,2] for example
        //    O => put current value
        //    (1,2) => same as n=2 case
        //    thus the num of valid patterns is
        //         case(n=0)*case(n=2)
        //                 1*        2
        //
        //    and the total nums = 2+1+2 = 5
        // total: 5

        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n as usize {
            for j in 1..=i {
                dp[i] += dp[j - 1] * dp[i - j];
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = 3;
        let except = 5;
        let output = Solution::num_trees(inputs);
        assert_eq!(except, output);
    }
}
