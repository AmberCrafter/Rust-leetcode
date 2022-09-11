// Too slow

pub struct Solution {}
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        // dp (includsive)
        // use upper triangle
        let s = s.chars().into_iter().collect::<Vec<char>>();
        let mut dp = vec![vec![false; s.len()]; s.len()];
        for i in 0..s.len() {
            // old elements
            let mut half_len = 0;
            let mut is_palindrome = true;
            while i>=half_len && i+half_len<s.len() && is_palindrome {
                for offset in 0..half_len+1 {
                    if s[i+offset]!=s[i-offset] {
                        is_palindrome = false;
                        break;
                    }
                    dp[i-offset][i+offset] = true;
                }
                half_len+=1;
            }

            // even elements
            half_len = 0;
            is_palindrome = true;
            while i>=half_len+1 && i+half_len<s.len() && is_palindrome {
                for offset in 0..half_len+1 {
                    if s[i-offset-1]!=s[i+offset] {
                        is_palindrome = false;
                        break;
                    }
                    dp[i-offset-1][i+offset] = true;
                }
                half_len+=1;
            }
        }

        // print dp table
        // for val in dp {
        //     println!("{:?}", val);
        // }

        // table is recorded the list of palindrome's end index (excludsive) 
        let mut table = vec![Vec::new(); s.len()];
        for (ri, row) in dp.iter().enumerate() {
            for (ci, col) in row.iter().enumerate() {
                if col==&true {
                    table[ri].push(ci+1);
                }
            }
        }

        // for val in &table {
        //     println!("table: {:?}", val);
        // }

        // check the shortest by dp 
        let mut dp = vec![i32::MAX; s.len()+1];
        dp[0] = 0;
        for (sp, members) in table.iter().enumerate() {
            for &ep in members {
                dp[ep] = dp[ep].min(dp[sp]+1);
            }
        }
        println!("dp: {:?}", dp);
        *dp.last().unwrap()-1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = "aab".to_string();
        let except = 1;
        let output = Solution::min_cut(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = "a".to_string();
        let except = 0;
        let output = Solution::min_cut(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = "ab".to_string();
        let except = 1;
        let output = Solution::min_cut(inputs);
        assert_eq!(except, output);
    }
}