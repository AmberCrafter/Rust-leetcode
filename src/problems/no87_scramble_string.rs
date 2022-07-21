pub struct Solution {}
impl Solution {
    fn greedy_split(s1: Vec<String>, target: String, counter: i32) -> bool {
        // println!("#{:}: {:?}", counter, s1);
        // if counter > 2 {
        //     return false;
        // }
        if s1.iter().map(|v| v.to_string()).collect::<String>() == target {
            return true;
        }

        // index notation
        // s1: i, j
        // s1 -> val: l
        for (i, val) in s1.iter().enumerate() {
            for l in 0..val.len()-1 {
                let mut next_stack = Vec::new();
                let mut next_stack_swap = Vec::new();
                for j in 0..s1.len() {
                    if i == j && val.len() > 1 {
                        let (p1, p2) = val.split_at(l+1);
                        next_stack.push(p1.to_string());
                        next_stack.push(p2.to_string());
                        next_stack_swap.push(p2.to_string());
                        next_stack_swap.push(p1.to_string());
                    } else {
                        next_stack.push(s1[j].to_string());
                        next_stack_swap.push(s1[j].to_string());
                    }
                }
                if Self::greedy_split(next_stack, target.to_string(), counter + 1)
                    || Self::greedy_split(next_stack_swap, target.to_string(), counter + 1)
                {
                    return true;
                }
            }
        }
        false
    }

    fn discussion_dp(s1: String, s2: String) -> bool {
        // ref https://leetcode.com/problems/scramble-string/discuss/2278190/Rust-Dynamic-Programming
        fn check(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, len: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            if dp[i][j][len] != -1 {return dp[i][j][len]};
            let mut same = true;
            for k in 0..len {
                if s1[i+k]==s2[j+k] {continue;}
                same = false;
                break;
            }
            if same {
                dp[i][j][len] = 1;
                return dp[i][j][len];
            }

            for k in 1..len {
                // swap case
                if check(s1,s2,i,j+len-k,k,dp)==1 && check(s1,s2,i+k,j,len-k,dp)==1 {
                    dp[i][j][len] = 1;
                    return dp[i][j][len];
                }

                // regular case
                if check(s1,s2,i,j,k,dp)==1 && check(s1,s2,i+k,j+k,len-k,dp)==1 {
                    dp[i][j][len]=1;
                    return dp[i][j][len];
                }
            }
            dp[i][j][len]=0;
            dp[i][j][len]
        }

        let n = s1.len();
        let s1 = s1.chars().collect::<Vec<char>>();
        let s2 = s2.chars().collect::<Vec<char>>();
        let mut dp = vec![vec![vec![-1; n+1];n];n];

        check(&s1, &s2, 0, 0, n, &mut dp) == 1
    }

    pub fn is_scramble(s1: String, s2: String) -> bool {
        // Self::greedy_split(vec![s1], s2, 0)
        Self::discussion_dp(s1, s2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ("great".to_string(), "rgeat".to_string());
        let except = true;
        let output = Solution::is_scramble(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = ("abcde".to_string(), "caebd".to_string());
        let except = false;
        let output = Solution::is_scramble(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = ("a".to_string(), "a".to_string());
        let except = true;
        let output = Solution::is_scramble(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = ("abcdbdacbdac".to_string(), "bdacabcdbdac".to_string());
        let except = true;
        let output = Solution::is_scramble(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = ("abb".to_string(), "bab".to_string());
        let except = true;
        let output = Solution::is_scramble(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
