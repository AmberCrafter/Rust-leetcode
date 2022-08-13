use std::collections::{HashMap, VecDeque};

pub struct Solution {}
impl Solution {
    // ref. https://leetcode.com/problems/minimum-window-substring/discuss/2142884/rust-solution
    pub fn min_window(s: String, t: String) -> String {
        let s_byte = s.as_bytes();
        let t_byte = t.as_bytes();
        let mut map = HashMap::<u8, i32>::new();
        let mut n_free = t.len() as i32;
        let (mut len, mut pos) = (s.len(), 0);
        // create target map
        for &b in t_byte {
            let count = map.entry(b).or_insert(0);
            *count += 1;
        }
        // create window to record target char index
        let mut window = VecDeque::<usize>::new();
        for (i, b) in s_byte.iter().enumerate() {
            if let Some(count) = map.get_mut(b) {
                *count -= 1;
                window.push_back(i);
                if *count >= 0 {
                    n_free -= 1;
                }

                // shrink left side
                while let Some(&idx) = window.front() {
                    let cur_count = map.get_mut(&s_byte[idx]).unwrap();
                    if *cur_count < 0 {
                        *cur_count += 1;
                        window.pop_front();
                    } else {
                        break;
                    }
                }

                // check and update minimum substring left index and length
                if n_free == 0 {
                    // this mean input string is valid
                    let cur_len = i - *window.front().unwrap();
                    if cur_len < len {
                        len = cur_len;
                        pos = i;
                    }
                }
            }
        }
        if len == s.len() {
            "".to_owned()
        } else {
            String::from(&s[pos - len..pos + 1])
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ("ADOBECODEBANC".to_string(), "ABC".to_string());
        let except = "BANC".to_string();
        let output = Solution::min_window(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = ("a".to_string(), "a".to_string());
        let except = "a".to_string();
        let output = Solution::min_window(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
    #[test]
    fn case3() {
        let inputs = ("a".to_string(), "aa".to_string());
        let except = "".to_string();
        let output = Solution::min_window(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}

/*
Note.
Need dp to record the pattern which be check?
<start target, end position>
*/
