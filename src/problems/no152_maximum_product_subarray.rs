pub struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut res = i32::MIN;
        for i in 0..nums.len() {
            for j in 0..i {
                nums[j] = nums[j] * nums[i];
                res = res.max(nums[j]);
            }
            res = res.max(nums[i]);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![2,3,-2,4];
        let except = 6;
        let output = Solution::max_product(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![-2, 0, -1];
        let except = 6;
        let output = Solution::max_product(inputs);
        assert_eq!(except, output);
    }
}