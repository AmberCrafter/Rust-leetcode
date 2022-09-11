use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // dp

        
        // map {
        //     sp: [ep]
        // }
        let s = s.bytes().collect::<Vec<u8>>();
        let mut map = HashMap::new();
        for word in word_dict {
            if word.len()>s.len() {break;}
            let limit = s.len()-word.len();
            for sp in 0..s.len() {
                if sp>limit {break;}
                let mut is_substring = true;
                for (offset, pat) in word.bytes().enumerate() {
                    if s[sp+offset] != pat {
                        is_substring = false;
                        break;
                    }
                }
                if is_substring {
                    let entry = map.entry(sp).or_insert(Vec::new());
                    entry.push(sp + word.len());
                }
            }
        }

        for val in &map {
            println!("map: {:?}", val);
        }

        fn walk(map: &HashMap<usize, Vec<usize>>, target: usize, idx: usize) -> bool {
            if idx == target {
                return true;
            }
            if let Some(elements) = map.get(&idx) {
                for &next in elements {
                    if walk(map, target, next) {
                        return true
                    }
                }
            }
            false
        }

        walk(&map, s.len(), 0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (
            "catsandog".to_string(),
            vec!["cats".to_string(),"dog".to_string(),"sand".to_string(),"and".to_string(),"cat".to_string()],
        );
        let except = false;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = (
            "cars".to_string(),
            vec!["car".to_string(),"ca".to_string(),"rs".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = (
            "aaaaaaa".to_string(),
            vec!["aaaa".to_string(),"aaa".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case6() {
        let inputs = (
            "bb".to_string(),
            vec!["a".to_string(),"b".to_string(),"bbb".to_string(),"bbbb".to_string()],
        );
        let except = true;
        let output = Solution::word_break(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}
