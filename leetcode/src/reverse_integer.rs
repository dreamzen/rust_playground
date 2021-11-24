struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        let mut digits = vec![];
        let mut v = x;
        while v != 0 {
            digits.push(v % 10);
            v /= 10;
        }

        let mut result = 0;

        for d in digits {
            if result > i32::MAX / 10 || result == i32::MAX / 10 && d > i32::MAX % 10 {
                return 0;
            }
            if result < i32::MIN / 10 || result == i32::MIN / 10 && d < i32::MIN % 10 {
                return 0;
            }
            result = result * 10 + d;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        // i32::MAX = 2147483647, i32::MIN = -2147483648
        assert_eq!(321, Solution::reverse(123));
        assert_eq!(-321, Solution::reverse(-123));
        assert_eq!(0, Solution::reverse(2147483647));
        assert_eq!(0, Solution::reverse(1563847412));
        assert_eq!(0, Solution::reverse(-2147483647));
    }
}
