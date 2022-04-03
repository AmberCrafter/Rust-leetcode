pub struct Solution {}

impl Solution {
    // ref: https://leetcode.com/problems/maximum-subarray/discuss/937879/Rust%3A-linear-solution
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut cur, mut max) = (nums[0], nums[0]);
        for i in 1..nums.len() {
            cur = match cur < 0 {
                true => nums[i],
                _ => cur + nums[i]
            };
            max = max.max(cur);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = [-2,1,-3,4,-1,2,1,-5,4].to_vec();
        let expected = 6;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = [1].to_vec();
        let expected = 1;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let inputs = [5,4,-1,7,8].to_vec();
        let expected = 23;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_4() {
        let inputs = [1,-1,1].to_vec();
        let expected = 1;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_5() {
        let inputs = [1,2,-1,-2,2,1,-2,1,4,-5,4].to_vec();
        let expected = 6;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_6() {
        let inputs = [-2,-1].to_vec();
        let expected = -1;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_7() {
        let inputs = [1,-3,2,0,-1,0,-2,-3,1,2,-3,2].to_vec();
        let expected = 3;
        let result = Solution::max_sub_array(inputs);

        assert_eq!(expected, result);
    }
}
