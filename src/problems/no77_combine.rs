pub struct Solution {}
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        /* n=6, k=4
        [1,2,3,4]
        [1,2,3,5]
        [1,2,3,6]
        [1,2,4,5]
        [1,2,4,6]
        [1,2,5,6]
        [1,3,4,5]
        ...
        [3,4,5,6]
        */

        let mut res = Vec::new();
        let mut tmp = (1..=k).into_iter().collect::<Vec<i32>>();
        res.push(tmp.clone());
        let mut count = 0;
        while tmp[0] < (n - k + 1) {
            let mut index = tmp.len() - 1;
            let mut bound = n;
            tmp[index] += 1;

            while tmp[index] > bound {
                bound = (tmp[index] - 1).min(n - k + index as i32);
                index -= 1;
                tmp[index] += 1;
            }

            while index < tmp.len() - 1 {
                tmp[index + 1] = tmp[index] + 1;
                index += 1;
            }
            res.push(tmp.clone());
        }
        // println!("{:?}", res);
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = (4, 2);
        let except = vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
        ];
        let output = Solution::combine(inputs.0, inputs.1);
        let mut except = except;
        except.sort();
        let mut output = output;
        output.sort();
        assert_eq!(except, output);
    }
}
