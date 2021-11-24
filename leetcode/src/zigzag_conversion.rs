struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() <= 1 || num_rows <= 1 {
            return s;
        }
        let mut matrics: Vec<Vec<char>> = Vec::new();
        for _ in 0..num_rows {
            matrics.push(Vec::new())
        }
        let mut index = 0;
        let mut op = 1;
        for c in s.chars() {
            matrics[index as usize].push(c);
            index += op;
            if index == num_rows - 1 || index == 0 {
                op *= -1;
            }
        }
        let mut result = String::new();
        for v in matrics {
            let x: String = v.iter().collect();
            result += &x;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            "PAHNAPLSIIGYIR".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 3)
        );
        assert_eq!(
            "PINALSIGYAHRPI".to_string(),
            Solution::convert("PAYPALISHIRING".to_string(), 4)
        );
        assert_eq!("A".to_string(), Solution::convert("A".to_string(), 4));
        assert_eq!("ABC".to_string(), Solution::convert("ABC".to_string(), 1));
    }
}
