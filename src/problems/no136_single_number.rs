// not pretty

pub struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        // check first
        if nums.len()==1 {
            return nums[0];
        }
        if nums[0]!=nums[1] {
            return nums[0];
        }


        for i in 1..nums.len()-1 {
            if nums[i]==nums[i-1] {
                continue;
            }
            if nums[i]==nums[i+1] {
                continue;
            }
            return nums[i];
        }
        *nums.last().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![2,2,1];
        let except = 1;
        let output = Solution::single_number(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![4,2,1,2,1];
        let except = 4;
        let output = Solution::single_number(inputs);
        assert_eq!(except, output);
    }
}