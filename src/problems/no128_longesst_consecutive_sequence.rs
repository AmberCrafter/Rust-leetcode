pub struct Solution {}
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        
        let mut acc = i32::MIN;
        
        let mut tmp = 0;
        let mut res = 0;

        for val in nums {
            if val<acc {continue;}
            if val==acc {
                tmp += 1;
                acc += 1;
            } else {
                acc = val+1;
                res = res.max(tmp);
                tmp = 1;
            }
        }
        res.max(tmp)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![100,4,200,1,3,2];
        let except = 4;
        let output = Solution::longest_consecutive(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![0,3,7,2,5,8,4,6,0,1];
        let except = 9;
        let output = Solution::longest_consecutive(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = vec![1,2,0,1];
        let except = 3;
        let output = Solution::longest_consecutive(inputs);
        assert_eq!(except, output);
    }
}