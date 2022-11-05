pub struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ').filter(|mem| mem!=&"").rev().collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = "the sky is blue".to_string();
        let except = "blue is sky the".to_string();
        let output = Solution::reverse_words(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = "  hello world  ".to_string();
        let except = "world hello".to_string();
        let output = Solution::reverse_words(inputs);
        assert_eq!(except, output);
    }
}