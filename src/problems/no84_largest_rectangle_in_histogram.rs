pub struct Solution {}
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // dp
        // table record maxiumn area and output the last value

        let lenght = heights.len() + 1;

        let mut area_table = vec![vec![0; lenght]; lenght];
        
        for r in 1..lenght {
            let mut h = heights[r-1];
            for c in r..lenght {
                h = h.min(heights[c-1]);
                area_table[r][c] = (h * (c - r + 1) as i32)
                    .max(area_table[r - 1][c])
                    .max(area_table[r][c - 1]);
            }
        }

        area_table.last().unwrap().last().unwrap().to_owned()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let inputs = vec![2, 1, 5, 6, 2, 3];
        let except = 10;
        let output = Solution::largest_rectangle_area(inputs);
        assert_eq!(except, output);
    }
}
