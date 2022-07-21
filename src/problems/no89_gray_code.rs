pub struct Solution {}
impl Solution {
    fn discussion_bitwise(n: i32) -> Vec<i32> {
        // ref. https://leetcode.com/problems/gray-code/discuss/2274033/C%2B%2BororGray-CodeororEasy-to-Understand
        let mut res = Vec::new();
        for i in 0..1<<n {
            res.push(i^(i>>1));
        }
        res
    }

    fn discussion_recursive(n: i32) -> Vec<i32> {
        // ref. https://leetcode.com/problems/gray-code/discuss/2271953/c%2B%2B-oror-RECURSIVE-Solution-Faster-than-82
        // base on or (|) operator and sequence relation
        // for example (n=3)
        // (init)   1-bit: 000, 001
        //          2-bit:      011, 010
        //                      011 = 001 | 010
        //                      010 = 000 | 010
        //          3-bit:           110, 111, 101, 000
        //                           110 = 010 | 100
        //                           111 = 011 | 100
        //                           101 = 001 | 100
        //                           100 = 000 | 100

        fn next(n: i32, shift: i32) -> Vec<i32> {
            if n==1 {
                return vec![0,1];
            }
            let mut res = next(n-1, shift-1);
            for i in (0..res.len()).rev() {
                res.push(res[i] | 1<<shift);
            }
            res
        }
        next(n, n-1)
    }

    pub fn gray_code(n: i32) -> Vec<i32> {
        // Self::discussion_bitwise(n)
        Self::discussion_recursive(n)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = 2;
        let except = vec![0,1,3,2];
        let output = Solution::gray_code(inputs);
        assert_eq!(except, output);
    }
}