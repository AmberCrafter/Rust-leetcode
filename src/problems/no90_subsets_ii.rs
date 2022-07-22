use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    fn recuse(res: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, map: &[(i32, i32)]) {
        if map.len()==0 {
            res.push(current.clone());
            return
        }

        for i in 0..=map[0].1 {
            if i!=0 {current.push(map[0].0)}
            Self::recuse(res, current, &map[1..]);
        }
        for i in 0..map[0].1 {
            current.pop();
        }
    }

    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(
            |&v| *map.entry(v).or_insert(0)+=1
        );

        let map = map.keys().into_iter().map(
            |key| (*key, *map.get(key).unwrap_or(&0))
        ).collect::<Vec<(i32, i32)>>();
        
        let mut res = Vec::new();
        let mut tmp = Vec::new();
        Solution::recuse(&mut res, &mut tmp, &map);
        println!("{:?}", res);
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![1,2,2];
        let except = vec![
            vec![],
            vec![2],
            vec![2,2],
            vec![1],
            vec![1,2],
            vec![1,2,2],
        ];
        let output = Solution::subsets_with_dup(inputs);
        assert_eq!(except, output);
    }
}