use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new(); // (slope, nums)
        let len = points.len();
        for i in 0..len {
            for j in i+1..len {
                let slope = (points[j].1-points[i].1) as f32 / (points[j].0-points[i].0) as f32;
                let entry = map.entry()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = None;
        let except = None;
        let output = None;
        assert_eq!(except, output);
    }
}