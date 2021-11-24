struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if s == p {
            return true;
        }
        if s.len() == 0 || p.len() == 0 {
            return false;
        }
        let s_str: Vec<char> = s.chars().collect();
        let p_str: Vec<char> = p.chars().collect();
        return sub_match(&s_str, 0, &p_str, 0);
    }
}

fn sub_match(s: &Vec<char>, s_pos: usize, p: &Vec<char>, p_pos: usize) -> bool {
    // match complete
    if s_pos == s.len() && p_pos == p.len() {
        return true;
    }
    // p is not enough
    if s_pos < s.len() && p_pos == p.len() {
        return false;
    }
    // s is not enough
    if s_pos == s.len() {
        // "" with "x*.."
        if p_pos + 1 < p.len() && p[p_pos + 1] == '*' {
            return sub_match(s, s_pos, p, p_pos + 2);
        }
        return false;
    }

    let mut is_match = false;
    // both s and p have remaining characters
    // p with *: ..x*...
    if p_pos + 1 < p.len() && p[p_pos + 1] == '*' {
        // * means zero
        is_match = is_match || sub_match(s, s_pos, p, p_pos + 2);
        // * means >= 1
        if s[s_pos] == p[p_pos] || p[p_pos] == '.' {
            is_match = is_match || sub_match(s, s_pos + 1, p, p_pos);
        }
        return is_match;
    } else {
        // p without x*
        if s[s_pos] == p[p_pos] || p[p_pos] == '.' {
            return sub_match(s, s_pos + 1, p, p_pos + 1);
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match() {
        assert_eq!(false, Solution::is_match("aa".to_string(), "a".to_string()));
        assert_eq!(true, Solution::is_match("aa".to_string(), "a.".to_string()));
        assert_eq!(true, Solution::is_match("aa".to_string(), "a*".to_string()));
        assert_eq!(
            true,
            Solution::is_match("aab".to_string(), "c*a*b".to_string())
        );
        assert_eq!(
            false,
            Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string())
        );
    }
}
