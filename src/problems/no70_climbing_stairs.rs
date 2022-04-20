pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // ref: https://leetcode.com/problems/climbing-stairs/discuss/740938/Rust-0ms-DP
        // ref: https://leetcode.com/problems/climbing-stairs/discuss/716499/Rust-Solution [best one]

        let mut buf = [2, 1];

        for i in 3..=n {
            buf[(i%2) as usize] = buf.into_iter().sum();
            // println!("{:?}", buf);
        }

        buf[(n%2) as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = 2;
        let except = 2;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = 3;
        let except = 3;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = 4;
        let except = 5;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = 5;
        let except = 8;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case5() {
        let inputs = 6;
        let except = 13;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case6() {
        let inputs = 7;
        let except = 21;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case7() {
        let inputs = 35;
        let except = 14930352;
        let output = Solution::climb_stairs(inputs);
        assert_eq!(except, output);
    }
}