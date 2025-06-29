use std::sync::LazyLock;
use std::sync::Mutex;

use crate::constants::*;
use crate::token::*;

type Int = usize;

pub struct Tokenizer {
    pub code: String,
    pub parse_index: Int,
}

impl Tokenizer {
    pub fn new(code: String, parse_index: Int) -> Self {
        Self { code, parse_index }
    }

    pub fn in_range(&self) -> bool {
        return self.parse_index < self.code.len();
    }

    pub fn about_to_hit_end(&self) -> bool {
        let mut peek_index = self.parse_index;
        while self.in_range() && self.current_char().is_ascii_whitespace() {
            peek_index += 1;
        }
        dbg!(peek_index);
        dbg!(self.parse_index);
        return peek_index == self.code.len();
    }

    pub fn current_char(&self) -> char {
        match self.code.chars().nth(self.parse_index){
            Some(c) => c,
            None => '\0',
        }
    }

    pub fn eat_spaces(&mut self) {
        while self.in_range() && (self.current_char() == ' ' || self.current_char() == '\t') {
            self.parse_index += 1;
        }
    }
    pub fn eat_all_spaces(&mut self) {
        while self.in_range() && SPACE_CHARS.contains(&self.current_char()) {
            self.parse_index += 1;
        }
    }

    pub fn peek_next_word(&mut self) -> String {
        let mut peek_index = self.parse_index;
        while self.in_range() && self.current_char().is_alphanumeric() {
            peek_index += 1;
        }
        return self.code[self.parse_index..peek_index].to_string();
    }

    pub fn next_in(&mut self, chars: &Vec<char>) -> String {
        let start = self.parse_index;
        while self.in_range() && chars.contains(&self.current_char()) {
            self.parse_index += 1;
        }
        return self.code[start..self.parse_index].to_string();
    }

    pub fn peek_next_in(&mut self, chars: &Vec<char>) -> &str {
        let mut peek_index = self.parse_index;
        while self.in_range() && chars.contains(&self.code.chars().nth(peek_index).unwrap()) {
            peek_index += 1;
        }
        return &self.code[self.parse_index..peek_index];
    }

    pub fn expect_char(&mut self, letter: char) {
        self.eat_all_spaces();
        assert!(self.in_range() && self.current_char() == letter);
        self.parse_index += 1;
    }

    pub fn optionaly_expect_char(&mut self, letter: char) -> bool {
        self.eat_spaces();
        if self.in_range() && self.current_char() == letter {
            self.parse_index += 1;
            return true;
        }
        return false;
    }

    pub fn next(&mut self) -> Token {
        self.eat_all_spaces();
        if !self.in_range() {return Token{type_:TokenType::EOF, value: "".to_string(), start_index:self.parse_index};}
        if self.current_char().is_numeric() {return Token{type_:TokenType::NUMBER, value: self.expect(TokenType::NUMBER), start_index:self.parse_index};}
        if self.current_char().is_alphabetic() {return Token{type_:TokenType::IDENTIFIER, value: self.expect(TokenType::IDENTIFIER), start_index:self.parse_index};}
        if self.current_char().is_ascii_whitespace() {self.eat_spaces(); return self.next();}
        if OPERATORS_TRIE.contains_letter(self.current_char()) {return Token{
            type_:TokenType::OPERATOR, value: self.expect(TokenType::OPERATOR), start_index:self.parse_index
        };}
        if self.current_char().is_ascii_punctuation() {return Token{type_:TokenType::PUNCTUATION, value: self.expect(TokenType::PUNCTUATION), start_index:self.parse_index};}
        panic!("not implemented");
        
    }
        pub fn expect(&mut self, type_: TokenType) -> String {
            println!("in expect ");
            dbg!(&type_);
            self.eat_all_spaces();
            let start = self.parse_index;
            match type_ {
                TokenType::NUMBER => {
                    while self.in_range() && self.current_char().is_numeric() {
                        self.parse_index += 1;
                    }
                }
                TokenType::IDENTIFIER => {
                    while self.in_range() && self.current_char().is_alphanumeric() {
                        self.parse_index += 1;
                    }
                }
                TokenType::STRING => {
                    panic!("not implemented");
                }
                TokenType::PUNCTUATION => {
                    let mut peek_index = self.parse_index;
                    while self.in_range() && self.current_char().is_ascii_punctuation() {
                        peek_index += 1;
                    }
                    let longest = OPERATORS_TRIE.greety(&self.code[start..peek_index]);
                    let len = longest.len();
                    self.parse_index += len;
                }
                TokenType::OPERATOR => {
                    let next_operator_stream = self.peek_next_in(&OPERATORS);
                    dbg!(&next_operator_stream);
                    let longest = OPERATORS_TRIE.greety(next_operator_stream);
                    let len = longest.len();
                    self.parse_index += len;
                }
                TokenType::KEYWORD => {
                    while self.in_range() && self.current_char().is_alphabetic() {
                        self.parse_index += 1;
                    }
                    let word = &self.code[start..self.parse_index];
                    assert!(KEYWORDS_TRIE.is_word(word));
                } // TokenType::DELIMITER => {
                //     while self.in_range() && self.current_char().is_alphanumeric() {
                //         self.parse_index += 1;
                //     }
                // },
                TokenType::EOF => {
                    assert!(self.parse_index == self.code.len());
                }
            }
            return self.code[start..self.parse_index].to_string();
    }
}
