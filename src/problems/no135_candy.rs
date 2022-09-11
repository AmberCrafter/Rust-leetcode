// too slow

pub struct Solution {}
impl Solution {
    // best
    // ref: https://leetcode.com/problems/candy/discuss/1300626/Rust-two-pass-solution
    // pub fn candy(ratings: Vec<i32>) -> i32 {
    //     let n = ratings.len();
    //     let mut ans = vec![1; n];
    //     for i in 1..n {
    //         if ratings[i] > ratings[i - 1] {
    //             ans[i] = ans[i - 1] + 1;
    //         }
    //     }
    //     for i in (0..(n - 1)).rev() {
    //         if ratings[i] > ratings[i + 1] {
    //             ans[i] = std::cmp::max(ans[i + 1] + 1, ans[i]);
    //         }
    //     }
    //     ans.into_iter().sum()
    // }



    pub fn candy(ratings: Vec<i32>) -> i32 {
        // list of given candy
        let mut candy = vec![0; ratings.len()];
        // list of the giving order index list
        let mut indexes = (0..ratings.len()).collect::<Vec<usize>>();
        indexes.sort_by_key(|&i| &ratings[i]);

        for &idx in &indexes {
            let mut num = 1;
            // check left
            if idx>0 && ratings[idx]>ratings[idx-1] {
                num = num.max(candy[idx-1]+1);
            }
            // check right
            if idx<indexes.len()-1 && ratings[idx]>ratings[idx+1] {
                num = num.max(candy[idx+1]+1);
            }

            candy[idx] = num;
        }
        candy.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1,0,2];
        let except = 5;
        let output = Solution::candy(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![1,2,2];
        let except = 4;
        let output = Solution::candy(inputs);
        assert_eq!(except, output);
    }
}