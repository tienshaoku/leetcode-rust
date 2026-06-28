use std::collections::HashMap;

#[derive(Default, Debug)]
struct Trie {
    children: HashMap<char, Trie>,
    is_last: bool,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        word.chars()
            .fold(self, |node, c| node.children.entry(c).or_default())
            .is_last = true;
    }

    fn get(&self, word: String) -> Option<&Trie> {
        word.chars().try_fold(self, |node, c| node.children.get(&c))
    }

    fn search(&self, word: String) -> bool {
        self.get(word).map_or(false, |n| n.is_last)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.get(prefix).is_some()
    }
}

#[derive(Default, Debug)]
struct TrieSlow {
    val: char,
    next: Vec<TrieSlow>,
    is_last: bool,
}

impl TrieSlow {
    fn new() -> Self {
        TrieSlow::default()
    }

    fn insert(&mut self, word: String) {
        if word.is_empty() {
            self.is_last = true;
            return;
        }
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        let rest: String = chars.collect();

        for child in &mut self.next {
            if child.val == first {
                child.insert(rest);
                return;
            }
        }

        let mut trie = TrieSlow {
            val: first,
            next: vec![],
            is_last: false,
        };
        trie.insert(rest);
        self.next.push(trie);
    }

    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return self.is_last;
        }

        let mut chars = word.chars();
        let first = chars.next().unwrap();
        let rest: String = chars.collect();

        for child in self.next.iter() {
            if child.val == first {
                return child.search(rest);
            }
        }
        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        if prefix.is_empty() {
            return true;
        }

        let mut chars = prefix.chars();
        let first = chars.next().unwrap();
        let rest: String = chars.collect();

        for child in self.next.iter() {
            if child.val == first {
                return child.starts_with(rest);
            }
        }
        false
    }
}

#[cfg(test)]
mod implement_trie_test {
    use super::*;

    #[test]
    fn implement_trie_test_1() {
        let mut trie = TrieSlow::new();
        trie.insert(String::from("apple"));
        assert_eq!(trie.search(String::from("apple")), true);
        assert_eq!(trie.search(String::from("app")), false);
        assert_eq!(trie.starts_with(String::from("app")), true);
        trie.insert(String::from("app"));
        assert_eq!(trie.search(String::from("apple")), true);
        assert_eq!(trie.search(String::from("app")), true);

        trie.insert(String::from("cough"));
        assert_eq!(trie.starts_with(String::from("co")), true);
        assert_eq!(trie.starts_with(String::from("col")), false);
        trie.insert(String::from("cold"));
        assert_eq!(trie.starts_with(String::from("col")), true);
    }
}
