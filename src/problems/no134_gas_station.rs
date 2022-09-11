// too slow

pub struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        // merge
        let table = (0..gas.len()).map(|idx| gas[idx] - cost[idx]).collect::<Vec<i32>>();
        let mut tmp = Vec::new();

        let mut sp = 0;
        let mut buf = 0;
        for (i, &value) in table.iter().enumerate() {
            if buf<0 {
                tmp.push((sp, buf));
                sp = i;
                buf = value;
                continue;
            }
            buf+=value;
        }
        // tmp.push((sp, buf));
        // println!("tmp: {:?}", tmp);
        if buf<0 {return -1;}
        
        for (_, val) in tmp {
            buf+=val;
            if buf<0 {
                return -1;
            }
        }
        sp as i32
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