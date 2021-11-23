use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut visited: HashMap<char, i32> = HashMap::new();
        let mut pre = 0;
        let mut max_len = 0;

        for (idx, c) in s.chars().enumerate() {
            if let Some(pos) = visited.get(&c) {
                if *pos >= pre {
                    pre = *pos + 1;
                } else {
                    max_len = max_len.max(idx as i32 - pre + 1);
                }
            } else {
                max_len = max_len.max(idx as i32 - pre + 1);
            }
            visited.insert(c, idx as i32);
        }
        max_len as i32
    }
}
