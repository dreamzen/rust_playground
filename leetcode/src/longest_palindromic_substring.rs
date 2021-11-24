struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return s;
        }
        let str: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = 0;

        // idx is usize, always >= 0!
        for idx in 0..str.len() {
            // ..aba..
            let mut l = idx;
            let mut r = idx;
            while l >= 0 && r < str.len() {
                if str[l] == str[r] {
                    if r - l > right - left {
                        left = l;
                        right = r;
                    }
                    // l is usize, cannot be -1!
                    // l -= 1;
                    if l == 0 {
                        break;
                    }
                    l -= 1;
                    r += 1;
                } else {
                    break;
                }
            }

            // ..abba..
            let mut l2 = idx;
            let mut r2 = idx + 1;
            while l2 >= 0 && r2 < str.len() {
                if str[l2] == str[r2] {
                    if r2 - l2 > right - left {
                        left = l2;
                        right = r2;
                    }
                    // l2 -= 1;
                    if l2 == 0 {
                        break;
                    }
                    l2 -= 1;
                    r2 += 1;
                } else {
                    break;
                }
            }
        }

        str[left..right + 1].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_longest() {
        assert_eq!(
            "aba".to_string(),
            Solution::longest_palindrome("aba".to_string())
        );
        assert_eq!(
            "abba".to_string(),
            Solution::longest_palindrome("abba".to_string())
        );
        assert_eq!("".to_string(), Solution::longest_palindrome("".to_string()));
        assert_eq!(
            "a".to_string(),
            Solution::longest_palindrome("a".to_string())
        );
        assert_eq!(
            "bb".to_string(),
            Solution::longest_palindrome("cbbd".to_string())
        );
        assert_eq!(
            "aca".to_string(),
            Solution::longest_palindrome("aacabdkacaa".to_string())
        );
    }
}
