pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // foward tacing
        let mut remain = 0;
        for value in nums {
            if remain<0 {return  false;}
            if value > remain {remain=value};
            remain-=1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = [2,3,1,1,4].to_vec();
        let expected = true;
        let result = Solution::can_jump(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = [3,2,1,0,4].to_vec();
        let expected = false;
        let result = Solution::can_jump(inputs);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_3() {
        let inputs = [0].to_vec();
        let expected = true;
        let result = Solution::can_jump(inputs);

        assert_eq!(expected, result);
    }
}
