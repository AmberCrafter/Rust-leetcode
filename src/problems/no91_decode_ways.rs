pub struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        fn check(s: &Vec<char>, idx: usize) -> bool {
            // decided current element can combine with previous element or not
            match s[idx - 1] {
                '1' => true,
                '2' => match s[idx] {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' => true,
                    _ => false
                },
                _ => false
            }
        }

        // dp solution
        let s = s.chars().into_iter().collect::<Vec<char>>();
        if s.len()==1 {
            return if s[0]!='0' {1} else {0}
        }

        // (# one char, # two char)
        let mut state = if s[0]=='0' {(0,0)} else {(1,0)};
        for i in 1..s.len() {
            state = if check(&s, i) {
                (if s[i]!='0' {state.0 + state.1} else {0}, state.0)
            } else {
                (if s[i]!='0' {state.0 + state.1} else {0}, 0)
            };
        }
        state.0 + state.1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = "12".to_string();
        let except = 2;
        let output = Solution::num_decodings(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = "226".to_string();
        let except = 3;
        let output = Solution::num_decodings(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case3() {
        let inputs = "06".to_string();
        let except = 0;
        let output = Solution::num_decodings(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case4() {
        let inputs = "1201234".to_string();
        let except = 3;
        let output = Solution::num_decodings(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case5() {
        let inputs = "10".to_string();
        let except = 1;
        let output = Solution::num_decodings(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case6() {
        let inputs = "00".to_string();
        let except = 0;
        let output = Solution::num_decodings(inputs);
        assert_eq!(except, output);
    }
}
