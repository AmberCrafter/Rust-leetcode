use core::slice;

pub struct Solution {}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // quick sort
        fn quicksort(array: &mut [i32]) {
            // [ unsorted | pivot | unsorted ]
            match array.len() {
                0 | 1 => return,
                2 => {
                    if array[0] > array[1] {
                        array.swap(0, 1);
                    }
                    return;
                }
                _ => {}
            }

            let (pivot, rest) = array.split_first_mut().expect("array is non-empty");
            let mut left = 0;
            let mut right = rest.len()-1;
            while left <= right {
                if &rest[left] <= pivot {
                    // already on the correct side
                    left += 1;
                } else if &rest[right]>pivot {
                    // right already on the correct side
                    // avoid unnessary swaps back and fort
                    if right==0 {
                        break;
                    }
                    right -= 1;
                } else {
                    // move element ot the right side
                    rest.swap(left, right);
                    left += 1;
                    if right==0 {
                        break;
                    }
                    right -= 1;
                }
            }

            let left = left +1;
            let right = right +1;

            // place the pivot at its final location
            array.swap(0, left-1);

            // split_at_mut(mid) -> (&mut [..mid], &mut[mid..])
            let (left, right) = array.split_at_mut(left-1);
            quicksort(left);
            quicksort(&mut right[1..]);
        }
    
        quicksort(nums);
    
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        let mut inputs = vec![2,0,2,1,1,0];
        let except = vec![0,0,1,1,2,2];
        let output = Solution::sort_colors(&mut inputs);
        assert_eq!(except, inputs);
    }
}
