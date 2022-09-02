pub struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let max_profit_vec = prices
            .iter()
            .rev()
            .scan(0, |state, &v| {
                *state = (*state).max(v);
                Some(*state)
            })
            .collect::<Vec<_>>();
        println!("{:?}", max_profit_vec);
        prices
			.iter()
			.zip(max_profit_vec.iter().rev())
			.skip(1)
			.fold(
            (0, prices[0], max_profit_vec[0] - prices[0]),
            |(max_so_far, min_so_far, result), (&v, &l)| {
			    let result = result.max(max_so_far + l - v);
			    let min_so_far = min_so_far.min(v);
			    let max_so_far = max_so_far.max(v - min_so_far);
                println!("{:?}\t{:?}",(v,l), (max_so_far, min_so_far, result));
			    (max_so_far, min_so_far, result)
		    },
        ).2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![3,3,5,0,0,3,1,4];
        let except = 6;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case2() {
        let inputs = vec![1,2,3,4,5];
        let except = 4;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case3() {
        let inputs = vec![7,6,4,3,1];
        let except = 0;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case4() {
        let inputs = vec![6,1,3,2,4,7];
        let except = 7;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }
    #[test]
    fn case5() {
        let inputs = vec![1,2];
        let except = 1;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }

    #[test]
    fn case6() {
        let inputs = vec![1,4,2];
        let except = 3;
        let output = Solution::max_profit(inputs);
        assert_eq!(except, output);
    }
}