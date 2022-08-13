pub struct Solution {}
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        // best solution is dp

        // check valied and parse
        fn check_parse(s: &[u8], spliter: [usize; 3]) -> Option<String> {
            let spliter = [0, spliter[0], spliter[1], spliter[2], s.len()];
            // check length
            for i in 0..spliter.len() - 1 {
                if spliter[i + 1] - spliter[i] > 3 {
                    return None;
                }
            }
            // println!("{:?}", spliter);

            let mut res = Vec::new();
            for i in 0..4 {
                if s[spliter[i]] == 48 && spliter[i + 1] - spliter[i] > 1 {
                    return None;
                } // invalid case which start with 0xx
                if spliter[i + 1] - spliter[i] == 3 {
                    if s[spliter[i]] > 50 {
                        return None;
                    } else if s[spliter[i]] == 50 {
                        if s[spliter[i] + 1] > 53 {
                            return None;
                        } else if s[spliter[i] + 1] == 53 {
                            if s[spliter[i] + 2] > 53 {
                                return None;
                            }
                        }
                    }
                }
                res.push(
                    std::str::from_utf8(&s[spliter[i]..spliter[i + 1]])
                        .unwrap()
                        .to_string(),
                );
            }
            Some(res.join("."))
        }

        // greedy methods
        // ptr struct:
        //  example. 1
        //      1234
        //       ^^^
        //      >> [1,2,3]
        //
        //  example. 2
        //      2552551113
        //         ^  ^^
        //      >> [3,6,7]

        let mut res = Vec::new();
        for i in 1..4 {
            if i >= s.len() {
                break;
            }
            for j in i + 1..i + 4 {
                if j >= s.len() {
                    break;
                }
                for k in j + 1..j + 4 {
                    if k >= s.len() {
                        break;
                    }
                    if let Some(ip) = check_parse(s.as_bytes(), [i, j, k]) {
                        res.push(ip);
                    }
                }
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
        let inputs = "25525511135".to_string();
        let except = vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()];
        let output = Solution::restore_ip_addresses(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = "0000".to_string();
        let except = vec!["0.0.0.0".to_string()];
        let output = Solution::restore_ip_addresses(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = "101023".to_string();
        let except = vec![
            "1.0.10.23".to_string(),
            "1.0.102.3".to_string(),
            "10.1.0.23".to_string(),
            "10.10.2.3".to_string(),
            "101.0.2.3".to_string(),
        ];
        let output = Solution::restore_ip_addresses(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = "172162541".to_string();
        let except = vec![
            "17.216.25.41".to_string(),
            "17.216.254.1".to_string(),
            "172.16.25.41".to_string(),
            "172.16.254.1".to_string(),
            "172.162.5.41".to_string(),
            "172.162.54.1".to_string(),
        ];
        let output = Solution::restore_ip_addresses(inputs);
        assert_eq!(except, output);
    }
}
