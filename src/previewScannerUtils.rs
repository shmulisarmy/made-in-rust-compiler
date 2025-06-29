use crate::token::TokenType;
use crate::tokenizer::Tokenizer;

use crate::expression::Expression;
pub fn looks_like_type(t: &mut Tokenizer)->bool {
    t.eat_spaces();
    if t.current_char().is_alphabetic() || t.current_char() == '_' {
        return true;
    }
    if t.current_char() == '[' {
        return true;
    }
    if t.current_char() == '(' {
        return true;
    }
    return false;
}

pub fn looks_like_expression(t: &mut Tokenizer)->bool {
    t.eat_spaces();
    if t.current_char().is_alphabetic() || t.current_char() == '_' {
        return true;
    }
    return false;
}

pub fn looks_like_identifier(t: &mut Tokenizer)->bool {
    t.eat_spaces();
    if t.current_char().is_alphabetic() || t.current_char() == '_' {
        return true;
    }
    return false;
}