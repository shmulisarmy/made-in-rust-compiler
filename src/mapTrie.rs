use std::collections::HashMap;

type NodeIndex = usize;

struct Node {
    children: HashMap<char, NodeIndex>,
    is_end: bool,
}

pub struct MapTrie {
    storage: Vec<Node>,
    root: NodeIndex,
}

impl MapTrie {
    pub fn new() -> Self {
        let mut res = MapTrie {
            storage: vec![],
            root: 0,
        };
        res.storage.push(Node {
            children: HashMap::new(),
            is_end: false,
        });
        res.root = res.storage.len() - 1;
        res
    }

    pub fn insert(&mut self, word: &String) {
        let mut current_node = self.root;
        for ch in word.chars() {
            let next = if let Some(&existing_next) = self.storage[current_node].children.get(&ch) {
                existing_next
            } else {
                let new_node_index = self.storage.len();
                self.storage.push(Node {
                    children: HashMap::new(),
                    is_end: false,
                });
                self.storage[current_node]
                    .children
                    .insert(ch, new_node_index);
                new_node_index
            };
            current_node = next;
        }
        self.storage[current_node].is_end = true;
    }

    pub fn is_word(&self, word: &String) -> bool {
        let mut current_node = self.root;
        for ch in word.chars() {
            match self.storage[current_node].children.get(&ch) {
                Some(&next) => current_node = next,
                None => return false,
            }
        }
        self.storage[current_node].is_end
    }

    pub fn contains(&self, word: &String) -> bool {
        let mut current_node = self.root;
        for ch in word.chars() {
            match self.storage[current_node].children.get(&ch) {
                Some(&next) => current_node = next,
                None => return false,
            }
        }
        true
    }

    pub fn contains_letter(&self, letter: char) -> bool {
        self.storage[self.root].children.contains_key(&letter)
    }

    pub fn greety(&self, word: &str) -> String {
        let mut current_node = self.root;
        let mut collected_letters = Vec::new();
        let chars: Vec<char> = word.chars().collect();

        for (i, &ch) in chars.iter().enumerate() {
            let next = self.storage[current_node].children.get(&ch);
            if next.is_none() {
                panic!("we don't have this word");
            }
            current_node = *next.unwrap();
            collected_letters.push(ch);

            if self.storage[current_node].is_end {
                if let Some(&next_ch) = chars.get(i + 1) {
                    if !self.storage[current_node].children.contains_key(&next_ch) {
                        return collected_letters.iter().collect();
                    }
                } else {
                    return collected_letters.iter().collect();
                }
            }
        }

        panic!("Looks like you're searching with a word that's not long enough");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        let mut t = MapTrie::new();
        assert!(!t.is_word(&"!=".to_string()));
        t.insert(&"!=".to_string());
        assert!(t.is_word(&"!=".to_string()));
        assert_eq!(t.greety(&"!==".to_string()), "!=");
        t.insert(&"!==".to_string());
        assert_eq!(t.greety(&"!==".to_string()), "!==");
    }
}
