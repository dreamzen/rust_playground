struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max_len = 0;
        let mut curr_len = 0;
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut pre = s.chars().nth(0).unwrap();
        for c in s.chars() {
            if c == pre {
                curr_len += 1;
            } else {
                pre = c;
                curr_len = 1;
            }
            max_len = max_len.max(curr_len);
        }

        max_len
    }
}
