use std::collections::HashMap;

use crate::expression::Expression;
use crate::expression::ExpressionPiece;
use crate::token::TokenType;
use crate::tokenizer::Tokenizer;
use crate::utils::red;

macro_rules! comp {
    [tuple_item_1:tt, tuple_item_2:tt; for x in expr] => {
        {
            let mut res = Vec::new();
            for x in expr {
                res.push((tuple_item_1, tuple_item_2));
            }
            res
        }
    };
    [$value:expr; until $cond:expr] => {
        {
            let mut res = Vec::new();
            while !$cond {
                res.push($value);
            }
            res
        }
    };

}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Field {
    name: String,
    type_: String,
    default_value: Expression,
}

impl Field {
    fn new(t: &mut Tokenizer) -> Self {
        let type_ = t.expect(TokenType::IDENTIFIER).to_string();
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        if t.optionaly_expect_char('=') {
            let default_value = Expression::new(t, '\n', '\n');
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value,
            };
        } else {
            t.eat_all_spaces();
            return Self {
                name,
                type_,
                default_value: Expression(ExpressionPiece::Placeholder(false)),
            };
        }
    }
}

pub struct Class {
    name: String,
    fields: Vec<Field>,
}

impl Class {
    pub fn new(t: &mut Tokenizer) -> Self {
        Self::preview_scan(t);
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        t.expect_char('{');
        t.eat_all_spaces();
        let fields = comp![Field::new(t); until t.optionaly_expect_char('}')];
        Self { name, fields }
    }

    pub fn display(&self) {
        println!("Class {} {{", self.name);
        for field in &self.fields {
            println!("    {} {}", field.type_, field.name);
        }
        println!("}}");
    }

    fn preview_scan(t: &mut Tokenizer) {
        use crate::previewScannerUtils::*;
        if !looks_like_identifier(t) {
            let next_token = t.next();
            t.user_error(
                next_token.start_index,
                next_token.start_index + next_token.value.len(),
            );
            panic!("{}", red("expected identifier (class name)".to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_parser() {
        let mut t = Tokenizer {
            file_name: file!(),
            start_line: line!() as usize,
            code: "class Person{
            int age
            string name
            string email
        }"
            .to_string(),
            parse_index: 0,
        };

        if t.expect(TokenType::KEYWORD) == "class" {
            let _class = Class::new(&mut t);
            _class.display();
            assert_eq!(_class.name, "Person");
            assert_eq!(_class.fields.len(), 3);

            assert_eq!(_class.fields[0].type_, "int");
            assert_eq!(_class.fields[0].name, "age");

            assert_eq!(_class.fields[1].type_, "string");
            assert_eq!(_class.fields[1].name, "name");

            assert_eq!(_class.fields[2].type_, "string");
            assert_eq!(_class.fields[2].name, "email");
        }
    }
}
