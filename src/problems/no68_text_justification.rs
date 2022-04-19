pub struct Solution {}
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let mut sp = 0;
        let mut idx = 0;

        while sp < words.len() {
            // get the range of the words push into the result vector
            let mut num_alphabet = 0;
            while (idx < words.len()) && ((num_alphabet + words[idx].len()) <= max_width as usize) {
                num_alphabet += words[idx].len();
                num_alphabet += 1; // for space
                idx += 1;
            }
            // println!("idx: {}\tnum_alphabet:{}", idx,num_alphabet);
            if idx == words.len() {
                let mut tmp = "".to_string();
                for i in sp..words.len() {
                    tmp.push_str(&words[i]);
                    tmp.push(' ');
                }
                match num_alphabet==(max_width+1) as usize {
                    true => {tmp.pop().unwrap();},
                    false => {tmp.push_str(
                        &((0..max_width as usize - num_alphabet).fold("".to_string(), |mut s, _| {
                            s.push(' ');
                            s
                        })),
                    );}
                }
                res.push(tmp);
                break;
            }

            // layout
            /*
            ex.
            max_width = 16

            "cccc__.cccc__ccc"

            c: charator
            _: base space
            .: additional space

            */

            // push the words into the result with padding space between the words
            let num_words = idx - sp;
            let total_space = max_width as usize + num_words - num_alphabet;
            let mut base = 0;
            let mut base_space = "".to_string();
            let mut addi = 0;

            match num_words {
                1 => {
                    base_space = (0..total_space).fold("".to_string(), |mut s, _| {
                        s.push(' ');
                        s
                    });
                }
                _ => {
                    // println!("total_space: {}",total_space);
                    base = total_space / (num_words - 1);
                    base_space = (0..base).fold("".to_string(), |mut s, _| {
                        s.push(' ');
                        s
                    });
                    addi = total_space % (num_words - 1);
                }
            }

            let mut tmp = "".to_string();
            for i in 0..num_words {
                tmp.push_str(&words[sp + i]);
                if i == num_words - 1 {
                    if i == 0 {
                        tmp.push_str(&base_space);
                    }
                    break;
                }
                tmp.push_str(&base_space);
                if i < addi {
                    tmp.push_str(" ")
                }
            }

            res.push(tmp);
            sp = idx;
            // println!("res: {:?}", res);
        }
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
