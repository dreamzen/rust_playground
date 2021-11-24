struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let str: Vec<char> = s.chars().collect();
        let mut is_negative = false;
        let mut start = 0usize;
        while start < s.len() && str[start] == ' ' {
            start += 1;
        }
        if start < s.len() && str[start] == '+' {
            start += 1;
        } else if start < s.len() && str[start] == '-' {
            start += 1;
            is_negative = true;
        }

        let mut result = 0;
        for idx in start..str.len() {
            if let Some(d) = str[idx].to_digit(10) {
                let d = d as i32;
                if !is_negative
                    && (result > i32::MAX / 10 || result == i32::MAX / 10 && d > i32::MAX % 10)
                {
                    return i32::MAX;
                }
                if is_negative
                    && (-result < i32::MIN / 10 || -result == i32::MIN / 10 && -d < i32::MIN % 10)
                {
                    return i32::MIN;
                }
                result = result * 10 + d;
            } else {
                break;
            }
        }
        return if is_negative { -result } else { result };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi() {
        assert_eq!(123, Solution::my_atoi("123".to_string()));
        assert_eq!(123, Solution::my_atoi("+123".to_string()));
        assert_eq!(-123, Solution::my_atoi("-123".to_string()));
        assert_eq!(0, Solution::my_atoi("-+123".to_string()));
        assert_eq!(4193, Solution::my_atoi("4193 with words".to_string()));
        assert_eq!(0, Solution::my_atoi("words and 987".to_string()));
        assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_string()));
        assert_eq!(2147483647, Solution::my_atoi("91283472332".to_string()));
        assert_eq!(0, Solution::my_atoi("".to_string()));
        assert_eq!(0, Solution::my_atoi("  ".to_string()));
        assert_eq!(0, Solution::my_atoi("  + ".to_string()));
        assert_eq!(-12, Solution::my_atoi("  -0012a42".to_string()));
    }
}
