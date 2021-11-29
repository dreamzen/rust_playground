struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut stack = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ch => {
                    if stack.len() == 0 {
                        return false;
                    }
                    match stack.pop().unwrap() {
                        '(' => {
                            if ch != ')' {
                                return false;
                            }
                        }
                        '[' => {
                            if ch != ']' {
                                return false;
                            }
                        }
                        '{' => {
                            if ch != '}' {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
            }
        }
        stack.len() == 0
    }
}
