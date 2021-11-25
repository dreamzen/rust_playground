struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }

        let mut next_idx = 0;
        while next_idx < strs[0].len() {
            let ch = strs[0].chars().nth(next_idx).unwrap();
            let mut is_match = true;
            for i in 1..strs.len() {
                if let Some(c) = strs[i].chars().nth(next_idx) {
                    if c != ch {
                        is_match = false;
                        break;
                    }
                } else {
                    is_match = false;
                    break;
                }
            }
            if !is_match {
                break;
            }
            next_idx += 1;
        }
        strs[0][..next_idx].to_string()
    }
}
