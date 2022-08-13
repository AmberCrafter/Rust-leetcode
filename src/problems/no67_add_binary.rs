pub struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut tmp = "".to_string();
        let mut ai = a.len();
        let mut bi = b.len();

        let mut a = a.as_bytes();
        let mut b = b.as_bytes();

        // carry value
        let mut c = 0;

        while (ai > 0) || (bi > 0) {
            let mut adder: u8 = 0;
            if ai > 0 {
                adder += match a[ai - 1] {
                    val => val - 48,
                    _ => 0,
                };
                ai -= 1;
            }

            if bi > 0 {
                adder += match b[bi - 1] {
                    val => val - 48,
                    _ => 0,
                };
                bi -= 1;
            }
            adder += c;

            tmp.push_str(&(adder % 2).to_string());
            c = adder / 2;
        }

        if c > 0 {
            tmp.push_str("1")
        };

        tmp.chars().rev().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = ("11".to_string(), "1".to_string());
        let except = "100".to_string();
        let result = Solution::add_binary(inputs.0, inputs.1);
        assert_eq!(except, result);
    }

    #[test]
    fn case2() {
        let inputs = ("1010".to_string(), "1011".to_string());
        let except = "10101".to_string();
        let result = Solution::add_binary(inputs.0, inputs.1);
        assert_eq!(except, result);
    }
}
