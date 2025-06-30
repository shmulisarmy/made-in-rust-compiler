type NodeIndex = usize;

struct Node {
    children: [NodeIndex; 128],
    is_end: bool,
}

pub struct Trie {
    storage: Vec<Node>,
    root: NodeIndex,
}

impl Trie {
    pub fn new() -> Self {
        let mut res = Trie {
            storage: vec![],
            root: 0,
        };
        res.storage.push(Node {
            children: [0; 128],
            is_end: false,
        });
        res.root = res.storage.len() - 1;
        res
    }
    pub fn insert(&mut self, word: &String) {
        let mut current_node = self.root;
        for char in word.chars() {
            let char_index = char as usize;
            if char_index >= 128 {
                panic!("Character '{}' is not ASCII", char);
            }
            if self.storage[current_node].children[char_index] == 0 {
                self.storage.push(Node {
                    children: [0; 128],
                    is_end: false,
                });
                self.storage[current_node].children[char_index] = self.storage.len() - 1;
            }
            current_node = self.storage[current_node].children[char_index];
        }
        self.storage[current_node].is_end = true;
    }

    pub fn is_word(&self, word: &str) -> bool {
        let mut current_node = self.root;
        for char in word.chars() {
            let char_index = char as usize;
            if char_index >= 128 {
                return false;
            }
            if self.storage[current_node].children[char_index] == 0 {
                return false;
            }
            current_node = self.storage[current_node].children[char_index];
        }
        return self.storage[current_node].is_end;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = self.root;
        for char in word.chars() {
            let char_index = char as usize;
            if char_index >= 128 {
                return false;
            }
            if self.storage[current_node].children[char_index] == 0 {
                return false;
            }
            current_node = self.storage[current_node].children[char_index];
        }
        return true;
    }

    pub fn contains_letter(&self, letter: char) -> bool {
        let char_index = letter as usize;
        if char_index >= 128 {
            return false;
        }
        self.storage[self.root].children[char_index] != 0
    }

    pub fn greety(&self, word: &str) -> String {
        //gets the biggest word it can going down the letters path
        let mut current_node = self.root;
        let mut collected_letters = Vec::new();
        for (index, char) in word.chars().enumerate() {
            let char_index = char as usize;
            if char_index >= 128 {
                panic!("Character '{}' is not ASCII", char);
            }
            if self.storage[current_node].children[char_index] == 0 {
                panic!("we dont have this word");
            }
            current_node = self.storage[current_node].children[char_index];
            collected_letters.push(char);
            if self.storage[current_node].is_end {
                let next_letter = word.chars().nth(index + 1);
                let should_continue_and_get_longer_word = next_letter.is_some() && {
                    let next_char = next_letter.unwrap();
                    let next_char_index = next_char as usize;
                    next_char_index < 128
                        && self.storage[current_node].children[next_char_index] != 0
                };
                if !should_continue_and_get_longer_word {
                    return collected_letters.iter().collect();
                }
            }
        }
        panic!("looks like your searching with a word thats not long enough");
    }
}
