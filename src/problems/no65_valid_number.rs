pub struct Solution {}

impl Solution {
    fn all_valid_char(s: &str) -> bool {
        let mut res = true;
        let mut exp_tag = false;
        for c in s.chars() {
            res &= match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
                '+' | '-' => true,
                'e' | 'E' => {
                    if !exp_tag {
                        exp_tag = true;
                        true
                    } else {
                        false
                    }
                }
                '.' => true,
                _ => false,
            };
        }
        res
    }

    fn is_correct_sign_position_and_float_point(s: &str) -> bool {
        let mut res = true;
        let mut float_point_tag = false;
        let mut has_digit = false;

        for (i, c) in s.chars().enumerate() {
            res &= match c {
                '+' | '-' => {
                    if i == 0 {
                        true
                    } else {
                        false
                    }
                }
                '.' => {
                    if !float_point_tag {
                        float_point_tag = true;
                        true
                    } else {
                        false
                    }
                }
                'e' | 'E' => false,
                _ => {
                    if has_digit {
                        true
                    } else {
                        has_digit = true;
                        true
                    }
                }
            };
        }
        res && has_digit
    }

    fn get_exp_position(s: &str) -> usize {
        s.find(['e', 'E']).unwrap_or(0)
    }

    pub fn is_number(s: String) -> bool {
        if !Solution::all_valid_char(&s) {
            return false;
        }

        let exp_position = Solution::get_exp_position(&s);

        if exp_position > 0 {
            Solution::is_correct_sign_position_and_float_point(&s[0..exp_position])
                && Solution::is_correct_sign_position_and_float_point(&s[exp_position + 1..s.len()])
                && s[exp_position + 1..s.len()].find('.').is_none()
        } else {
            Solution::is_correct_sign_position_and_float_point(&s)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = "0".to_string();
        let except = true;
        let result = Solution::is_number(inputs);
        assert_eq!(except, result);
    }
    #[test]
    fn case2() {
        let inputs = "e".to_string();
        let except = false;
        let result = Solution::is_number(inputs);
        assert_eq!(except, result);
    }
    #[test]
    fn case3() {
        let inputs = ".".to_string();
        let except = false;
        let result = Solution::is_number(inputs);
        assert_eq!(except, result);
    }

    #[test]
    fn case4() {
        let inputs = "6e6.5".to_string();
        let except = false;
        let result = Solution::is_number(inputs);
        assert_eq!(except, result);
    }
}
