pub struct Solution {}
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // ref. https://leetcode.com/problems/interleaving-string/discuss/1247457/Rust-DP-solution
        // view:
        // s1: 123
        // s2: 24567
        // s3: 12324567
        //
        // s2 sized stack:[t, _, _, _, _, _]
        // s2            :    2  4  5  6  7
        // s1            :    1  2  3
        // index         : 0  1  2  3  4  5

        
        if s1.len()+s2.len() != s3.len() {return false}
        // dp method
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![false; s2.len()+1];
        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                let u = if i+j>0 {s3[i+j-1]} else {0};
                dp[j] = match(i,j) {
                    // (take the nth char from s1, take the nth char from s2) which the char index start at 1
                    (0,0) => true,
                    (0,_) => dp[j-1] && s2[j-1]==u,
                    (_,0) => dp[j] && s1[i-1]==u,
                    _ => (dp[j-1] && s2[j-1]==u) || (dp[j] && s1[i-1]==u)
                }
            }
        }
        dp[s2.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string(),
        );
        let except = true;
        let output = Solution::is_interleave(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string(),
        );
        let except = false;
        let output = Solution::is_interleave(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = ("a".to_string(), "b".to_string(), "a".to_string());
        let except = false;
        let output = Solution::is_interleave(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = ("abababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string(), "babababababababababababababababababababababababababababababababababababababababababababababababaaaba".to_string(), "abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababbb".to_string());
        let except = false;
        let output = Solution::is_interleave(inputs.0, inputs.1, inputs.2);
        assert_eq!(except, output);
    }
}
