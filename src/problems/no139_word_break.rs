use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // dp
        let s = s.as_bytes();
        let nums = s.len();
        let mut dp = vec![false; nums+1];
        dp[nums] = true;
        for i in (1..nums+1).rev() {
            if dp[i]==false {continue;}
            for word in word_dict.iter() {
                let len = word.len();
                if i<len {continue;}
                if word.as_bytes()==&s[(i-len)..i] {
                    dp[i-len] = true && dp[i];
                }
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (
            "catsandog".to_string(),
            vec!["cats".to_string(),"dog".to_string(),"sand".to_string(),"and".to_string(),"cat".to_string()],
        );
        let except = false;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (
            "cars".to_string(),
            vec!["car".to_string(),"ca".to_string(),"rs".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = (
            "aaaaaaa".to_string(),
            vec!["aaaa".to_string(),"aaa".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case6() {
        let inputs = (
            "bb".to_string(),
            vec!["a".to_string(),"b".to_string(),"bbb".to_string(),"bbbb".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
