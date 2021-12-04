use std::collections::HashMap;

struct StreamChecker {
    map: HashMap<String, bool>,
    suffixes: Vec<String>,
}

// TLE.. Need to use Trie to solve this problem
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for word in words.iter() {
            map.insert(word.clone(), true);
        }
        StreamChecker {
            map,
            suffixes: vec![String::new()],
        }
    }

    fn query(&mut self, letter: char) -> bool {
        let mut ss = vec![letter.to_string(); self.suffixes.len() + 1];
        for (i, s) in self.suffixes.iter().enumerate() {
            ss[i] = s.clone() + &ss[i];
        }
        self.suffixes = ss;
        for s in self.suffixes.iter() {
            if self.map.contains_key(s) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() {
        let mut stream_checker =
            StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
        assert_eq!(false, stream_checker.query('a'));
        assert_eq!(false, stream_checker.query('c'));
        assert_eq!(true, stream_checker.query('d'));
        assert_eq!(true, stream_checker.query('f'));
        assert_eq!(false, stream_checker.query('k'));
        assert_eq!(true, stream_checker.query('l'));
        assert_eq!(false, stream_checker.query('a'));
    }
}
