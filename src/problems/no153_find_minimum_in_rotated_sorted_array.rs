pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // check rotated
        if &nums[0]<nums.last().unwrap() {return nums[0];}
        // binary search
        let mut l = 0;
        let mut r = nums.len();
        while l+1<r {
            let (ll, rr) = (l, r);
            let mid = (l+r)/2;
            if nums[ll]>nums[mid] {r = mid+1;}
            if nums[mid]>=nums[rr-1] {l = mid;}
        }
        nums[l]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![3,4,5,1,2];
        let except = 1;
        let output = Solution::find_min(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case2() {
        let inputs = vec![4,5,6,7,0,1,2];
        let except = 0;
        let output = Solution::find_min(inputs);
        assert_eq!(except, output);
    }
}