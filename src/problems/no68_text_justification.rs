use core::num;

pub struct Solution {}
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let mut nums_char = 0;
        let mut sp = 0;
        let mut tmp_str = "".to_string();

        for (i, s) in words.iter().enumerate() {
            if nums_char + s.len() + i - sp <= max_width as usize {
                nums_char += s.len();
                continue;
            }

            let space_num = max_width as usize - nums_char;
            let mut space = "".to_string();
            tmp_str.clear();

            // space_num/(i-sp-1) + (j<space_num%(i-sp-1) ? 1 : 0)

            tmp_str.push_str(&words[sp]);
            for j in sp + 1..i {
                tmp_str.push_str({
                    space = match i - sp {
                        0 => "".to_string(),
                        1 => (0..space_num).fold("".to_string(), |mut buf, _| {
                            buf.push(' ');
                            buf
                        }),
                        _ => match j - sp - 1 < space_num % (i - sp - 1) {
                            true => (0..space_num / (i - sp - 1) + 1).fold(
                                "".to_string(),
                                |mut buf, _| {
                                    buf.push(' ');
                                    buf
                                },
                            ),
                            false => {
                                (0..space_num / (i - sp - 1)).fold("".to_string(), |mut buf, _| {
                                    buf.push(' ');
                                    buf
                                })
                            }
                        },
                    };
                    &space
                });
                tmp_str.push_str(&words[j]);
            }

            // padding space for inline singal word
            tmp_str.push_str(&(tmp_str.len()..max_width as usize).fold(
                "".to_string(),
                |mut buf, _| {
                    buf.push(' ');
                    buf
                },
            ));

            // record the formatted string
            res.push(tmp_str.to_string());

            // set next iter initial metadata
            nums_char = s.len();
            sp = i;
        }

        tmp_str.clear();
        for s in words[sp..].into_iter() {
            if tmp_str.len() > 0 {
                tmp_str.push(' ');
            }
            tmp_str.push_str(s);
        }
        tmp_str.push_str(&(tmp_str.len()..max_width as usize).fold(
            "".to_string(),
            |mut buf, _| {
                buf.push(' ');
                buf
            },
        ));
        res.push(tmp_str);
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            vec![
                "This".to_string(),
                "is".to_string(),
                "an".to_string(),
                "example".to_string(),
                "of".to_string(),
                "text".to_string(),
                "justification.".to_string(),
            ],
            16,
        );
        let except = vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string(),
        ];
        let output = Solution::full_justify(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (
            vec![
                "What".to_string(),
                "must".to_string(),
                "be".to_string(),
                "acknowledgment".to_string(),
                "shall".to_string(),
                "be".to_string(),
            ],
            16,
        );
        let except = vec![
            "What   must   be".to_string(),
            "acknowledgment  ".to_string(),
            "shall be        ".to_string(),
        ];
        let output = Solution::full_justify(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (
            vec![
                "Science".to_string(),
                "is".to_string(),
                "what".to_string(),
                "we".to_string(),
                "understand".to_string(),
                "well".to_string(),
                "enough".to_string(),
                "to".to_string(),
                "explain".to_string(),
                "to".to_string(),
                "a".to_string(),
                "computer.".to_string(),
                "Art".to_string(),
                "is".to_string(),
                "everything".to_string(),
                "else".to_string(),
                "we".to_string(),
                "do".to_string(),
            ],
            20,
        );
        let except = vec![
            "Science  is  what we".to_string(),
            "understand      well".to_string(),
            "enough to explain to".to_string(),
            "a  computer.  Art is".to_string(),
            "everything  else  we".to_string(),
            "do                  ".to_string(),
        ];
        let output = Solution::full_justify(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (
            vec![
                "ask".to_string(),
                "not".to_string(),
                "what".to_string(),
                "your".to_string(),
                "country".to_string(),
                "can".to_string(),
                "do".to_string(),
                "for".to_string(),
                "you".to_string(),
                "ask".to_string(),
                "what".to_string(),
                "you".to_string(),
                "can".to_string(),
                "do".to_string(),
                "for".to_string(),
                "your".to_string(),
                "country".to_string(),
            ],
            16,
        );
        let except = vec![
            "ask   not   what".to_string(),
            "your country can".to_string(),
            "do  for  you ask".to_string(),
            "what  you can do".to_string(),
            "for your country".to_string(),
        ];
        let output = Solution::full_justify(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
