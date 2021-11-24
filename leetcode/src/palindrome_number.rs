struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        let mut digit: Vec<i32> = vec![];
        let mut v = x;
        while v != 0 {
            digit.push(v % 10);
            v /= 10;
        }
        let mut left = 0;
        let mut right = digit.len() - 1;
        while left <= right {
            if digit[left as usize] == digit[right as usize] {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }
        true
    }
}
