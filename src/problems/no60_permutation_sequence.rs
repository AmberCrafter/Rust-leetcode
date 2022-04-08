pub struct Solution {}

impl Solution {
    // ref. https://www.cnblogs.com/grandyang/p/4358678.html
    /* note. 
    ex.
    n = 5
    k = 6
    arr = [1,2,3,4,5]
    res = [_,_,_,_,_]
    
    ----------------------------------------------------------------------------------------------------
    由於此permutation method 會優先調整陣列尾部的數字，因此前頭的數字會存在連續出現的次數，可由排列組合判定關係
    [1,_,_,_,_]
     ^
    第一位fix的次數為，
        第二位有 4 種可能
        第三位有 3 種可能
        第四位有 2 種可能
        第五位有 1 種可能
    因此第一位的重複次數為24次

    [n,2,_,_,_]
    第二位fix的次數為，
        第三位有 3 種可能
        第四位有 2 種可能
        第五位有 1 種可能
    因此第二位的重複次數為6次

    [n,n,3,_,_]
    第三位fix的次數為，
        第四位有 2 種可能
        第五位有 1 種可能
    因此第三位的重複次數為2次

    [n,n,n,4,_]
    第四位fix的次數為，
        第五位有 1 種可能
    因此第四位的重複次數為1次

    [n,n,n,n,5]
    第五位的重複次數為1次

    該fix位所填入的值，可以透過殘餘的陣列元素[a,b,c,d]進行判斷，例如
    該位重複次數為6次，選取第17位排序結果，
    a 排序 6 次，共  6 次
    b 排序 6 次，共 12 次
    c 排序 5 次，共 17 次
    故可得知填入值為c
    

    以此循環往覆，可以判斷在特定排列序號下，各個欄位的數值。
    */

    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut v: Vec<char> = (0..n as u8).map(|i| (b'1' + i) as char).collect();
        let mut k: i32 = k - 1;
        let mut m: i32 = (1..n).product();
        let mut answer: String = String::with_capacity(n as usize);
        for i in 0..n - 1 {
            answer.push(v.remove((k / m) as usize));
            k %= m;
            m /= n - i - 1;
        }
        answer.push(v[0]);
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let inputs = (3,3);
        let expected = "213".to_string();
        let result = Solution::get_permutation(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    #[test]
    fn case_2() {
        let inputs = (4,9);
        let expected = "2314".to_string();
        let result = Solution::get_permutation(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }

    
    #[test]
    fn case_3() {
        let inputs = (3,1);
        let expected = "123".to_string();
        let result = Solution::get_permutation(inputs.0, inputs.1);

        assert_eq!(expected, result);
    }
}
