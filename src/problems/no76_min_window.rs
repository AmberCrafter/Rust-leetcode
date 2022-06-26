use std::collections::HashMap;

pub struct Solution {}

type HashTable = HashMap<char, usize>;

impl Solution {
    fn find_right_bound(table: &HashTable, s: &str) -> usize {
        if table.contains_key(&s.chars().nth(0).unwrap()) {
            let mut table = table.clone();
            for (i, c) in s.chars().into_iter().enumerate() {
                if table.contains_key(&c) {
                    if *table.get(&c).unwrap()>1 {
                        *table.get_mut(&c).unwrap()-=1;
                    } else {
                        table.remove(&c);
                    }
                }
    
                if table.keys().len()==0 {
                    return i;
                }
            }
        }
        usize::MAX
    }

    pub fn min_window(s: String, t: String) -> String {
        // get target hashtable
        let mut table: HashTable = HashMap::new();  // (desire, current)
        for c in t.chars().into_iter() {
            *table.entry(c).or_insert(0) += 1;
        }

        // truncate string
        let left = s.chars().into_iter().position(|c| table.contains_key(&c)).unwrap();
        let right = s.len()-s.chars().into_iter().rev().position(|c| table.contains_key(&c)).unwrap();

        let s = &s[left..right];

        let mut dp: Vec<usize> = Vec::new();
        if let Some(n) = s.len().checked_sub(t.len()) {
            for i in 0..n+1 {
                dp.push(Solution::find_right_bound(&table, &s[i..]));
            }

            let &target = dp.iter().min().unwrap();
            if target<=s.len() {
                let left = dp.iter().position(|&x| x==target).unwrap();
                let right = left+target+1;
                return (&s[left..right]).to_string()
            }
        }
        "".to_string()
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