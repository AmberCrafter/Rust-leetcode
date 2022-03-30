pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        // bitwise method on index
        let mut x = x;
        let mut n = n;
        let mut res = 1.0;

        if n < 0 {
            if n==i32::MIN {
                res /= x;
                n+=1;
            }


            x = 1.0 / x;
            n = !n + 1;
        }

        while n > 0 {
            if n & 1 == 1 {
                res *= x
            }
            x *= x;
            n >>= 1;
        }
        
        (res*(1e5) as f64).round() * (1e-5) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = (5.0, 2);
        let expected = 25.0;
        let result = Solution::my_pow(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = (2.1, 3);
        let expected = 9.261;
        let result = Solution::my_pow(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let inputs = (2.0, -2);
        let expected = 0.25;
        let result = Solution::my_pow(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_4() {
        let inputs = (0.00001, 2147483647);
        let expected = 0.0;
        let result = Solution::my_pow(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_5() {
        let inputs = (2.0, -2147483648);
        let expected = 0.0;
        let result = Solution::my_pow(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_6() {
        let inputs = (34.00515, -3);
        let expected = 3e-5;
        let result = Solution::my_pow(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }
}
