pub struct Solution {}
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        // dp
        // table is recorded the list of palindrome's end index (excludsive) 
        let s = s.chars().into_iter().collect::<Vec<char>>();
        let mut table = vec![Vec::new(); s.len()];
        for st in 0..s.len() {
            for ed in st+1..=s.len() {
                // check palindrome
                let mut is_palindrome = true;
                let half_len = (ed-st)/2;
                for offset in 0..half_len {
                    if s[st+offset]!=s[ed-offset-1] {
                        is_palindrome = false;
                        break;
                    }
                }
                if is_palindrome {
                    table[st].push(ed);
                }
            }
        }

        // check the shortest by recursive
        fn walk(result: &mut Vec<Vec<String>>, table: &Vec<Vec<usize>>, s: &Vec<char>, buf: &mut Vec<String>, index: usize) {
            if index==s.len() {
                result.push(buf.to_vec());
                return;
            }
            for &idx in &table[index] {
                buf.push(s[index..idx].iter().map(|&c| c).collect::<String>());
                walk(result, table, s, buf, idx);
                buf.pop();
            }
        }
        let mut buf = Vec::new();
        let mut result = Vec::new();
        walk(&mut result, &table, &s, &mut buf, 0);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = "aab".to_string();
        let except = vec![vec!["a","a","b"],vec!["aa","b"]];
        let output = Solution::partition(inputs);
        assert_eq!(except, output);
    }
}