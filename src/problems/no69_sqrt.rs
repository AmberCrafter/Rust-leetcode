pub struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 => 0,
            1 => 1,
            _ => {
                // binary search
                let mut left = 0;
                let mut right = x;
                let mut mid = (left + right) / 2;

                while mid > x / mid || x / (mid + 1) >= (mid + 1) {
                    if mid <= x / mid {
                        left = mid
                    } else {
                        right = mid
                    }
                    mid = (left + right) / 2;
                }
                mid
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = 4;
        let except = 2;
        let output = Solution::my_sqrt(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = 8;
        let except = 2;
        let output = Solution::my_sqrt(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = 89995;
        let except = 299;
        let output = Solution::my_sqrt(inputs);
        assert_eq!(except, output);
    }
}
