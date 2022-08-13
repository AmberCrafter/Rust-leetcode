pub struct Solution {}
impl Solution {
    // best solution: https://leetcode.com/problems/distinct-subsequences/discuss/236467/Rust-Bottom-Up-DP-with-0-ms-and-O(N)-space
    pub fn num_distinct(s: String, t: String) -> i32 {
        let slen = s.len();
        let tlen = t.len();

        // consider s and t are ascii string
        let s = s.as_bytes();
        let t = t.as_bytes();

        // dp
        let mut dp = vec![vec![0; s.len()+1]; t.len()+1];

        // init 
        (1..slen+1).for_each(|i| dp[0][i] = 1);

        // iteration
        let mut repeat_times = 0;
        for j in 1..tlen+1 {
            for i in dp[j-1][0]+1..slen+1 {
                if s[i-1]==t[j-1] {
                    if j>1 && t[j-1]==t[j-2] {
                        // repeat case
                        dp[j][i] = dp[j-1][i-1] + dp[j][i-1];
                    } else {
                        // raguler case
                        dp[j][i] = dp[j-1][i] + dp[j][i-1];
                    }
                        
                    if dp[j][0]==0 {
                        dp[j][0] = i;
                    }
                } else {
                    dp[j][i] = dp[j][i-1];
                }
            }
        }

        print!("       ");
        for i in 0..slen {
            print!("{:?}", std::str::from_utf8(&[s[i]]).unwrap());
        }
        print!("\n");
        for (i,val) in dp.iter().enumerate() {
            if i>0 {
                print!("{:?} ", std::str::from_utf8(&[t[i-1]]).unwrap())
            }else {
                print!("    ");
            }
            println!("{:?}", val);
        }
        dp[tlen][slen] as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ("rabbbit".to_string(), "rabbit".to_string());
        let except = 3;
        let output = Solution::num_distinct(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = ("babgbag".to_string(), "bag".to_string());
        let except = 5;
        let output = Solution::num_distinct(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = ("ddd".to_string(), "dd".to_string());
        let except = 3;
        let output = Solution::num_distinct(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
    #[test]
    fn case4() {
        let inputs = ("eee".to_string(), "eee".to_string());
        let except = 1;
        let output = Solution::num_distinct(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
    #[test]
    fn case5() {
        let inputs = ("aabb".to_string(), "abb".to_string());
        let except = 2;
        let output = Solution::num_distinct(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}