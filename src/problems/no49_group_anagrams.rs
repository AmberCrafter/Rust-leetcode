pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let mut arr: Vec<char> = s.chars().collect();
            arr.sort();
            let key: String = arr.into_iter().collect();
            map.entry(key)
                .and_modify(|x| x.push(s.clone()))
                .or_insert(vec![s]);
        }

        map.into_iter()
            .map(|(_, value)| value)
            .collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = vec!["abc".to_string(), "edf".to_string(), "bac".to_string()];
        let expected = vec![
            ["abc".to_string(), "bac".to_string()].to_vec(),
            ["edf".to_string()].to_vec(),
        ];
        let result = Solution::group_anagrams(inputs);
        assert_eq!(expected, result);
    }
}
