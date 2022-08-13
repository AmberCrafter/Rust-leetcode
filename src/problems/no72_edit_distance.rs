pub struct Solution {}
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // ref. https://leetcode.com/problems/edit-distance/discuss/2171978/All-Dynamic-Programming-Solutions-in-progressive-manner-(memo-tabulation-space-optimised)
        // tabulation
        // 規則:
        // 1. r==l: 0(不用操作)+左上角(先前步驟)
        // 2. r!=l: 1(replace)+最少步驟(左上角[先前步驟], 左側[delete], 上側[Insert])

        let n1 = word1.len();
        let n2 = word2.len();

        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];

        // set insert step
        for i in 1..=n2 {
            dp[0][i] = i;
        }

        // set delete step
        for i in 1..=n1 {
            dp[i][0] = i;
        }

        // calc total step
        for i in 1..=n1 {
            for j in 1..=n2 {
                if word1.chars().nth(i - 1) == word2.chars().nth(j - 1) {
                    dp[i][j] = 0 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i][j - 1].min(dp[i - 1][j]));
                }
            }
        }
        dp[n1][n2] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ("hours".to_string(), "ros".to_string());
        let except = 3;
        let output = Solution::min_distance(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = ("intention".to_string(), "execution".to_string());
        let except = 5;
        let output = Solution::min_distance(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
