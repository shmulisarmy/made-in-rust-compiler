use core::panic;
use std::process;
use std::sync::LazyLock;
use std::sync::Mutex;

use crate::constants::*;
use crate::project_basic_utils::token::*;
use crate::utils::blue;
use crate::utils::red;

type Int = usize;

pub struct Tokenizer {
    pub mutex: Mutex<()>, /**
        the assumption is that when you call some kind of tokenizing function you already have the tokenizer unlocked
    */
    pub file_name: &'static str,
    pub start_line: Int,
    pub code: &'static str,
    pub parse_index: Int,
}

impl Tokenizer {
    pub fn in_range(&self) -> bool {
        return self.parse_index < self.code.len();
    }

    pub fn about_to_hit_end(&self) -> bool {
        let mut peek_index = self.parse_index;
        while self.in_range() && self.current_char().is_ascii_whitespace() {
            peek_index += 1;
        }
        //dbg!(peek_index);
        //dbg!(self.parse_index);
        return peek_index == self.code.len();
    }

    pub fn current_char(&self) -> char {
        match self.code.as_bytes().get(self.parse_index) {
            Some(&byte) => byte as char,
            None => '\0',
        }
    }
    pub fn next_char(&self) -> char {
        match self.code.as_bytes().get(self.parse_index+1) {
            Some(&byte) => byte as char,
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
            if self.looks_like_at_comment() {
                self.eat_comment();
            }
        }
    }


    pub fn looks_like_at_comment(&self) -> bool {
        self.in_range() && self.current_char() != '/' && self.next_char() == '/'
    }

    pub fn eat_comment(&mut self) {
        while self.in_range() && self.current_char() != '\n' {
            self.parse_index += 1;
        } 
    }

    pub fn peek_next_word(&mut self) -> &str {
        self.eat_all_spaces();
        let mut peek_index = self.parse_index;
        while self.in_range() && self.code.as_bytes()[peek_index].is_ascii_alphabetic() {
            peek_index += 1;
        }
        return &self.code[self.parse_index..peek_index];
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
        while self.in_range() && chars.contains(&(self.code.as_bytes()[peek_index] as char)) {
            peek_index += 1;
        }
        return &self.code[self.parse_index..peek_index];
    }

    pub fn expect_char(&mut self, letter: char) {
        self.eat_all_spaces();
        //dbg!(self.current_char());
        if !self.in_range() {
            panic!("your at the end of the file in a position where you still need to parse");
        }
        if self.current_char() != letter {
            self.user_error(self.parse_index, self.parse_index + 1);
            panic!("expected {} but got {}", letter, self.current_char());
        }
        self.parse_index += 1;
    }

    pub fn expect_char_with_backups(&mut self, letter: char, backups: &[char; 1]) {
        //the assumption is that letter is the one that will get eaten and if so the tokenizer will consume that char, if not it will look at the backups first but not step forward
        self.eat_all_spaces();
        //dbg!(self.current_char());
        if !self.in_range() {
            panic!("your at the end of the file in a position where you still need to parse");
        }
        if self.current_char() == letter {
            self.parse_index += 1;
            return;
        }
        for backup in backups {
            if self.current_char() == *backup {
                return;
            }
        }
        self.user_error(self.parse_index, self.parse_index + 1);
        let formated_backups = backups
            .iter()
            .map(|c| format!("{}", c))
            .collect::<Vec<String>>()
            .join(", ");
        panic!(
            "expected {} or any of the following: {} but got {}",
            letter,
            formated_backups,
            self.current_char()
        );
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
        let token_start = self.parse_index;
        if !self.in_range() {
            return Token {
                type_: TokenType::EOF,
                value: "",
                start_index: token_start,
            };
        }

        if self.current_char().is_numeric() {
            return Token {
                type_: TokenType::NUMBER,
                value: self.expect(TokenType::NUMBER),
                start_index: token_start,
            };
        }
        if self.current_char().is_alphabetic() {
            return Token {
                type_: TokenType::IDENTIFIER,
                value: self.expect(TokenType::IDENTIFIER),
                start_index: token_start,
            };
        }
        if self.current_char().is_ascii_whitespace() {
            self.eat_spaces();
            return self.next();
        }
        if OPERATORS_TRIE.contains_letter(self.current_char()) {
            return Token {
                type_: TokenType::OPERATOR,
                value: self.expect(TokenType::OPERATOR),
                start_index: self.parse_index,
            };
        }

        //its important that this run before the .is_ascii_punctuation check bc quotes are considered punctuation and will therefore be caught by the .is_ascii_punctuation check and it will parse it incorrectly
        if self.current_char() == '\'' {
            self.parse_index += 1;
            let start_index = self.parse_index;
            while self.in_range() && self.current_char() != '\'' {
                self.parse_index += 1;
            }
            let token = Token {
                type_: TokenType::STRING,
                value: &self.code[start_index..self.parse_index],
                start_index: self.parse_index,
            };
            self.parse_index += 1; // skip the closing quote, (if not the next thing that tries to parse will end up thinking that the rest of the file is part of that string)
            return token;
        }
        if PUNCTUATION_TRIE.contains_letter(self.current_char()) {
            // panic!("punctuation trie contains {}", self.current_char());
            let next_punc_len =
                PUNCTUATION_TRIE.the_most_we_can_collect_on_word(self.peek_until_space());
            let token = Token {
                value: &self.code[self.parse_index..self.parse_index + next_punc_len],
                type_: TokenType::PUNCTUATION,
                start_index: self.parse_index,
            };
            //dbg!(next_punc_len);
            self.parse_index += next_punc_len;
            return token;
        }

        //its important that this run before the .is_ascii_punctuation check bc quotes are considered punctuation and will therefore be caught by the .is_ascii_punctuation check and it will parse it incorrectly
        if self.current_char() == '"' {
            self.parse_index += 1;
            let start_index = self.parse_index;
            while self.in_range() && self.current_char() != '"' {
                self.parse_index += 1;
            }
            let token = Token {
                type_: TokenType::STRING,
                value: &self.code[start_index..self.parse_index],
                start_index: self.parse_index,
            };
            self.parse_index += 1; // skip the closing quote, (if not the next thing that tries to parse will end up thinking that the rest of the file is part of that string)
            return token;
        }
        if self.current_char().is_ascii_punctuation() {
            assert_ne!(self.current_char(), '\'', "quotes should be handled above");
            assert_ne!(self.current_char(), '"', "quotes should be handled above");
            return Token {
                type_: TokenType::PUNCTUATION,
                value: self.expect(TokenType::PUNCTUATION),
                start_index: self.parse_index,
            };
        }

        panic!("not implemented");
    }
    pub fn expect(&mut self, type_: TokenType) -> &'static str {
        //dbg!(&self.parse_index);
        //dbg!(&self.current_char());
        //dbg!(&type_);
        self.eat_all_spaces();
        if self.current_char() == ';' {
            self.user_error(self.parse_index, self.parse_index + 1);
            println!(
                "{}",
                red(
                    "in this language we dont use semicolons (this is a modern language)"
                        .to_string()
                )
            );
            panic!("stack trace view");
        }
        let start = self.parse_index;
        match type_ {
            TokenType::NUMBER => {
                while self.in_range() && self.current_char().is_numeric() {
                    self.parse_index += 1;
                }
            }
            TokenType::IDENTIFIER => {
                while self.in_range()
                    && (self.current_char().is_alphanumeric() || self.current_char() == '_')
                {
                    self.parse_index += 1;
                }
                //dbg!(start);
                //dbg!(self.parse_index);
                //dbg!(&self.code[start..self.parse_index]);
                if start == self.parse_index {
                    let next_token = &self.next();
                    self.user_error(
                        next_token.start_index,
                        next_token.start_index + next_token.value.len(),
                    );
                    println!("dont see a valid identifier");
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
                println!("longest: {}", longest);
                let len = longest.len();
                self.parse_index += len;
            }
            TokenType::OPERATOR => {
                let next_operator_stream = self.peek_next_in(&OPERATORS);
                //dbg!(&next_operator_stream);
                let next_operator_len =
                    OPERATORS_TRIE.the_most_we_can_collect_on_word(next_operator_stream);
                //dbg!(next_operator_len);
                self.parse_index += next_operator_len;
            }
            TokenType::KEYWORD => {
                while self.in_range() && self.current_char().is_alphabetic() {
                    self.parse_index += 1;
                }
                let word = &self.code[start..self.parse_index];
                if !KEYWORDS_TRIE.is_word(word) {
                    println!("{} is not a keyword", word);
                    if !KEYWORDS_TRIE.is_word(word) {
                        self.user_error(start, self.parse_index);
                        // Removed to fix mutable/immutable borrow issue
                        panic!("expected a keyword but got {}", word);
                    }
                }
            } // TokenType::DELIMITER => {
            //     while self.in_range() && self.current_char().is_alphanumeric() {
            //         self.parse_index += 1;
            //     }
            // },
            TokenType::EOF => {
                assert!(self.parse_index == self.code.len());
            }
        }
        if start == self.parse_index {
            self.user_error(self.parse_index, self.parse_index + 1);
        } 
        return &self.code[start..self.parse_index];
    }

    pub fn peek_until_space(&mut self) -> &str {
        let mut peek_index = self.parse_index;
        while self.in_range() && !self.current_char().is_ascii_whitespace() {
            peek_index += 1;
        }
        return &self.code[self.parse_index..peek_index];
    }

    //ui methods
    pub fn user_error(&self, start_index: Int, end_index: Int) {
        println!(
            "{}\x1b[31;4m{}\x1b[0m{}",
            &self.code[..start_index],
            &self.code[start_index..end_index],
            &self.code[end_index..]
        );
        let (line, column) = self.find_line_and_column(end_index);
        let error_location_link =
            format!("{}:{}:{}", self.file_name, line, column);
        println!(
            "{} {}",
            red("error".to_string()),
            blue(&error_location_link)
        );
        std::process::exit(1);
    }

    //ui methods
    pub fn find_line_and_column(&self, start_index: Int) -> (Int, Int) {
        let mut line = self.start_line;
        let mut column = 1;
        for i in 0..start_index {
            if self.code.as_bytes()[i] == b'\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        return (line, column);
    }
}
