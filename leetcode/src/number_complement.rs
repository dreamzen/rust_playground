struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut num = num;
        while num != 0 {
            digits.push(num % 2);
            num /= 2;
        }

        let mut result = 0;
        for i in (0..digits.len()).rev() {
            let n = if digits[i] == 0 { 1 } else { 0 };
            result = result * 2 + n;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_complement() {
        Solution::find_complement(5);
    }
}
