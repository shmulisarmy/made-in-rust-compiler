use std::collections::HashMap;
use std::task::Context;

use crate::parser::expression::Expression;
use crate::parser::expression::ExpressionPiece;
use crate::project_basic_utils::token::*;
use crate::project_basic_utils::tokenizer::*;
use crate::utils::red;

use crate::comp;
use crate::until;
use crate::SyntaxNode;
use crate::PARSE_CONTEXT;

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



// 1) define really well what iv done in the class::new method and have the ai implement that change in others
// 2) start searching up the tree when a type is seen

impl Class {
    pub fn new(t: &mut Tokenizer, parser_context: &mut Vec<SyntaxNode>) -> Self {
        Self::preview_scan(t);
        let name = t.expect(TokenType::IDENTIFIER).to_string();
        t.expect_char('{');
        t.eat_all_spaces();
        parser_context.push(SyntaxNode::Class(Class{ name, fields: Vec::new() }));
        let idx = parser_context.len() - 1;
        if let SyntaxNode::Class(c) = &mut parser_context[idx] {
            c.fields = comp![Field::new(t); until t.optionaly_expect_char('}')];
        }
        // Pop after parsing body and return owned value
        if let SyntaxNode::Class(c) = parser_context.pop().unwrap() {
            c
        } else {
            unreachable!("Expected Class node on context stack")
        }
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
            code: "class Person{\n            int age\n            string name\n            string email\n        }"
            .to_string(),
            parse_index: 0,
        };

        if t.expect(TokenType::KEYWORD) == "class" {
            let mut context = vec![];
            let class_val = Class::new(&mut t, &mut context);
            class_val.display();
            assert_eq!(class_val.name, "Person");
            assert_eq!(class_val.fields.len(), 3);

            assert_eq!(class_val.fields[0].type_, "int");
            assert_eq!(class_val.fields[0].name, "age");

            assert_eq!(class_val.fields[1].type_, "string");
            assert_eq!(class_val.fields[1].name, "name");

            assert_eq!(class_val.fields[2].type_, "string");
            assert_eq!(class_val.fields[2].name, "email");
        }
    }
}
