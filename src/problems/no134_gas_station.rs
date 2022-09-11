// TLE

pub struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for st in 0..gas.len() {
            let mut count = 0;
            
            // first half
            for s in st..gas.len() {
                count=count + gas[s] - cost[s];
                if count<0 {
                    break;
                }
            }
            if count<0 {continue;}

            // second half
            for s in 0..st {
                count=count + gas[s] - cost[s];
                if count<0 {
                    break;
                }
            }
            if count<0 {continue;}
            return st as i32;
        }
        -1
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
}