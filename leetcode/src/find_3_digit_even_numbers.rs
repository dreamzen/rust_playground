struct Solution;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut digits = digits;
        digits.sort();

        for i in 0..digits.len() {
            if digits[i] == 0 {
                continue;
            }
            if i > 0 && digits[i] == digits[i - 1]{
                continue;
            }
            let mut last_j_value = 10;
            for j in 0..digits.len() {
                if j == i {
                    continue;
                }
                if digits[j] == last_j_value {
                    continue;
                }
                last_j_value = digits[j];
                for k in 0..digits.len() {
                    if k == i || k == j {
                        continue;
                    }
                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                    if num % 2 != 0 {
                        continue;
                    }
                    if result.len() == 0 || result[result.len() - 1] != num {
                        result.push(num);
                    }
                }
            }
        }
        result
    }
}
