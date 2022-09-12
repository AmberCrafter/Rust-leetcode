pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn walk(result: &mut Vec<String>, buffer: &mut Vec<String>, s: &str, word_dict: &Vec<String>) {
            if s.len()==0 {
                result.push(buffer.join(" "));
            }

            for word in word_dict.iter() {
                let len = word.len();
                if s.len()>=len && word==&s[0..len] {
                    buffer.push(s[0..len].to_string());
                    walk(result, buffer, &s[len..], word_dict);
                    buffer.pop();
                }
            }
        }


        let mut buf = Vec::new();
        let mut res = Vec::new();

        walk(&mut res, &mut buf, &s, &word_dict);
        res

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            "catsanddog".to_string(),
            vec!["cat".to_string(),"cats".to_string(),"and".to_string(),"sand".to_string(),"dog".to_string()]
        );
        let except = vec!["cat sand dog".to_string(),"cats and dog".to_string()];
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}