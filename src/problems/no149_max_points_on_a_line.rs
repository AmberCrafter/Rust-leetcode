use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let len = points.len();
        let mut res = 1;
        for i in 0..len {
            for j in i+1..len {
                let mut count = 2;
                let x1 = points[i][0];
                let y1 = points[i][1];
                let x2 = points[j][0];
                let y2 = points[j][1];
                
                for k in 0..len {
                    if k==i || k==j {continue;}
                    if (y2-y1)*(points[k][0] - x1) == (points[k][1]-y1)*(x2-x1) {
                        count += 1;
                    }
                }
                res = res.max(count);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![vec![1,1],vec![2,2],vec![3,3]];
        let except = 3;
        let output = Solution::max_points(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![vec![1,1],vec![3,2],vec![5,3],vec![4,1],vec![2,3],vec![1,4]];
        let except = 4;
        let output = Solution::max_points(inputs);
        assert_eq!(except, output);
    }
    
}