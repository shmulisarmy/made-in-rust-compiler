
use core::str;

type Int = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    IDENTIFIER,
    KEYWORD,
    NUMBER,
    STRING,
    // CHARACTER,
    PUNCTUATION,
    OPERATOR,
    // DELIMITER,
    EOF,
}


#[derive(Debug, Clone)]
pub struct Token {
    pub type_: TokenType,
    pub value: String,
    pub start_index: Int,
}