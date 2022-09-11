// too slow

pub struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut pidx = 0;
        let mut nval = 0;
        let mut pval = 0;

        for idx in 0..gas.len() {
            if pval<0 {
                nval+=pval;
                pval = 0;
                pidx = idx;
            }
            pval += gas[idx] - cost[idx];
        }

        if pval<0 || pval+nval<0 {
            -1
        } else {
            pidx as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (vec![1,2,3,4,5], vec![3,4,5,1,2]);
        let except = 3;
        let output = Solution::can_complete_circuit(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = (vec![2,3,4], vec![3,4,3]);
        let except = -1;
        let output = Solution::can_complete_circuit(inputs.0, inputs.1);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = (vec![4], vec![5]);
        let except = -1;
        let output = Solution::can_complete_circuit(inputs.0, inputs.1);
        assert_eq!(except, output);
    }
}   