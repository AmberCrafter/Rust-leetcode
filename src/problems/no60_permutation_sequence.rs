pub struct Solution {}

impl Solution {
    pub fn permute_one<T>(nums: &Vec<T>) -> Vec<T> 
    where 
        T: PartialEq + PartialOrd + Copy + std::fmt::Debug,
    {
        /*
        example: 
                        nums = [2 ,1, 6, 4, 2]
        step 1: find the decrease point from the end of list
                                   ^
                                   dp
           
        step 2: swap the dp value with the larger one which figure out from the end of list
                                            ^
                                            swap target
        step 3: reverse the part of the list after dp
                [2, 2, 6, 4, 1]  -> [2, 1, 4, 6, 2]
                       ^
                       reverse  start point
        */


        let nums_len = nums.len();
        
        let mut nums = (*nums).clone();
        if nums_len==0 {return nums;}
        // println!("Input: {:?}", nums);

        // step 1: find the decrease point
        let mut dp = 0;
        let mut flag = true;
        for i in 1..nums_len{
            // println!("Step1: i:{:?}\tx:{:?}\ty:{:?}", i, nums[nums_len-i], nums[nums_len-i-1]);
            if nums[nums_len-i]>nums[nums_len-i-1] {
                dp = nums_len-i-1;
                flag = false;
                break;
            }
        };

        // println!("Step1: {:?}\t {:?}", dp, nums);
        if dp==0 && flag{
            nums.reverse(); return nums
        };

        // step 2: swap the value which larger than dp
        for i in 1..nums_len-dp {
            if nums[nums_len-i]>nums[dp] {nums.swap(nums_len-i, dp); break;}
        }
        // println!("Step2: {:?}", nums);

        // println!("dp: {}", dp);
        // step 3: reverse the last parts of the list after dp
        for i in 1..=(nums_len-dp)/2 {
            nums.swap(nums_len-i, dp+i);
            // println!("Step3: {:?}", nums);
        }
        nums
    }

    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut res = (1..=n).map(|v| v).collect::<Vec<i32>>();
        for _ in 1..k {
            res = Solution::permute_one(&res);
        }
        res.into_iter().map(|v| v.to_string()).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = (3,3);
        let expected = "213".to_string();
        let result = Solution::get_permutation(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = (4,9);
        let expected = "2314".to_string();
        let result = Solution::get_permutation(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    
    #[test]
    fn case_3() {
        let inputs = (3,1);
        let expected = "123".to_string();
        let result = Solution::get_permutation(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }
}
