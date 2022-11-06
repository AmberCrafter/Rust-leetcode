pub struct Solution {}
impl Solution {
    /* best solution
    ref: https://leetcode.com/problems/compare-version-numbers/discuss/1798018/Rust-solution

    pub fn compare_version(version1: String, version2: String) -> i32 {
        use std::cmp::Ordering::{Greater, Less};
        
        let (mut it_1, mut it_2) = (version1.split('.'), version2.split('.'));
        loop {
            let (s1, s2) = (it_1.next(), it_2.next());
            if s1.is_none() && s2.is_none() {
                break;
            }

            let s1 = s1.unwrap_or("0").trim_start_matches('0');
            let s2 = s2.unwrap_or("0").trim_start_matches('0');
            if s1.len() != s2.len() {
                return (s1.len() as i32 - s2.len() as i32).signum();
            }

            for (c1, c2) in s1.chars().zip(s2.chars()) {
                match c1.cmp(&c2) {
                    Greater => return 1,
                    Less => return -1,
                    _ => (),
                }
            }
        }
        0
    }
    */

    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1_iter = version1.split('.');
        let mut v2_iter = version2.split('.');
        let mut res = 0;
        while let (v1, v2) = (v1_iter.next(), v2_iter.next()) {
            match (v1, v2) {
                (Some(v1), Some(v2)) => {
                    let v1 = i32::from_str_radix(v1, 10).unwrap();
                    let v2 = i32::from_str_radix(v2, 10).unwrap();
                    if v1>v2 {return 1;}
                    if v1<v2 {return -1;}
                },
                (Some(v1), None) => {
                    if i32::from_str_radix(v1, 10).unwrap()==0 {
                        res = 0;
                    } else {
                        return 1;
                    }
                },
                (None, Some(v2)) => {
                    if i32::from_str_radix(v2, 10).unwrap()==0 {
                        res = 0;
                    } else {
                        return -1;
                    }
                },
                (None, None) => return 0
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ("1.01".to_string(), "1.001".to_string());
        let except = 0;
        let output = Solution::compare_version(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = ("1.0".to_string(), "1.0.0".to_string());
        let except = 0;
        let output = Solution::compare_version(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = ("0.1".to_string(), "1.1".to_string());
        let except = -1;
        let output = Solution::compare_version(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}