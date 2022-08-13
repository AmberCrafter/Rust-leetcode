pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if res > 0 {
                    break;
                }
            } else {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = "Hello World".to_string();
        let expected = 5;
        let result = Solution::length_of_last_word(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = "   fly me   to   the moon  ".to_string();
        let expected = 4;
        let result = Solution::length_of_last_word(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let inputs = "luffy is still joyboy".to_string();
        let expected = 6;
        let result = Solution::length_of_last_word(inputs);

        assert_eq!(expected, result);
    }
}
