use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let hash_map: HashMap<char, &str> = [
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "pqrs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]
        .iter()
        .cloned()
        .collect();

        let mut prefixes: Vec<String> = Vec::new();

        for d in digits.chars() {
            if let Some(str) = hash_map.get(&d) {
                let mut res: Vec<String> = Vec::new();
                for c in str.chars() {
                    if prefixes.len() == 0 {
                        res.push(c.to_string());
                    } else {
                        for prefix in prefixes.iter() {
                            let mut p = prefix.clone();
                            p.push(c);
                            res.push(p);
                        }
                    }
                }
                prefixes = res;
            }
        }

        prefixes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            vec![] as Vec<String>,
            Solution::letter_combinations("".to_string())
        );
    }
}
