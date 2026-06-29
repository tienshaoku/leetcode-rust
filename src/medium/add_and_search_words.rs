use std::collections::HashMap;

#[derive(Default, Debug)]
struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary::default()
    }

    fn add_word(&mut self, word: String) {
        word.chars()
            .fold(self, |node, c| node.children.entry(c).or_default())
            .is_end = true;
    }

    fn search(&self, word: String) -> bool {
        self.search_str(&word)
    }

    fn search_str(&self, word: &str) -> bool {
        if word.is_empty() {
            return self.is_end;
        }

        let (first, rest) = word.split_at(1);
        let c = first.chars().next().unwrap();

        if c == '.' {
            self.children.values().any(|child| child.search_str(rest))
        } else {
            match self.children.get(&c) {
                Some(child) => child.search_str(rest),
                None => false,
            }
        }
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[cfg(test)]
mod add_and_search_words_test {
    use super::*;

    #[test]
    fn add_and_search_words_test_1() {
        let mut word_dictionary = WordDictionary::new();
        word_dictionary.add_word(String::from("bad"));
        word_dictionary.add_word(String::from("dad"));
        word_dictionary.add_word(String::from("mad"));

        assert_eq!(word_dictionary.search(String::from("pad")), false);
        assert_eq!(word_dictionary.search(String::from("bad")), true);
        assert_eq!(word_dictionary.search(String::from(".ad")), true);
        assert_eq!(word_dictionary.search(String::from("b..")), true);
    }
}
