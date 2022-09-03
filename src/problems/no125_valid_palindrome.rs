pub struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // consider only ascii value in s
        let s = s
            .as_bytes()
            .iter()
            .filter(|&&c| c.is_ascii_alphanumeric())
            .map(|&c| c.to_ascii_lowercase())
            .collect::<Vec<u8>>();
        let num = s.len();
        for i in 0..num / 2 {
            if s[i] != s[num - i - 1] {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = "A man, a plan, a canal: Panama".to_string();
        let except = true;
        let output = Solution::is_palindrome(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = "race a car".to_string();
        let except = false;
        let output = Solution::is_palindrome(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = "0P".to_string();
        let except = false;
        let output = Solution::is_palindrome(inputs);
        assert_eq!(except, output);
    }
}
